// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use bytes::{BufMut, BytesMut};
use futures::{Poll, Async};
use futures::stream::Stream;
use hyper::{self, Request};
use hyper::header::{ContentDisposition, ContentType, DispositionParam, DispositionType, Header};
use hyper::mime::{self, Mime};
use rand::{self, Rng};
use std::borrow::Borrow;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, Cursor, Read, Write};
use std::iter::{FromIterator, Peekable};
use std::path::Path;
use std::str::FromStr;
use std::vec::IntoIter;


/// Converts a hyper Header into a String.
///
fn header_to_string<H>(header: &H) -> String
where
    H: Header + Display,
{
    format!("{}: {}", H::header_name(), header)
}


/// Writes a CLRF.
///
fn write_crlf<W>(write: &mut W) -> io::Result<()>
where
    W: Write,
{
    write.write_all(&[b'\r', b'\n'])
}


/// Multipart body that is compatible with Hyper.
///
pub struct Body {
    /// The amount of data to write with each chunk.
    ///
    buf_size: usize,

    /// The active reader.
    ///
    current: Option<Box<'static + Read + Send>>,

    /// The parts as an iterator. When the iterator stops
    /// yielding, the body is fully written.
    ///
    parts: Peekable<IntoIter<Part>>,

    /// The multipart boundary.
    ///
    boundary: String,
}

impl Body {
    /// Implements section 4.1.
    ///
    /// [See](https://tools.ietf.org/html/rfc7578#section-4.1).
    ///
    fn write_boundary<W>(&self, write: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        write_crlf(write)?;
        write.write_all(&[b'-', b'-'])?;
        write.write_all(self.boundary.as_bytes())
    }

    fn write_final_boundary<W>(&self, write: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        self.write_boundary(write)?;
        write.write_all(&[b'-', b'-'])
    }

    /// Writes the Content-Disposition, and Content-Type headers.
    ///
    fn write_headers<W>(&self, write: &mut W, part: &Part) -> io::Result<()>
    where
        W: Write,
    {
        write_crlf(write)?;
        write.write_all(
            header_to_string(&part.content_type).as_bytes(),
        )?;
        write_crlf(write)?;
        write.write_all(
            header_to_string(&part.content_disposition).as_bytes(),
        )?;
        write_crlf(write)?;
        write_crlf(write)
    }
}

impl Stream for Body {
    type Item = BytesMut;

    type Error = hyper::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let bytes = BytesMut::with_capacity(self.buf_size);
        let mut writer = bytes.writer();

        if self.current.is_none() {
            if let Some(part) = self.parts.next() {
                self.write_boundary(&mut writer)?;
                self.write_headers(&mut writer, &part)?;

                let read = match part.inner {
                    Inner::Read(read, _) => read,
                    Inner::Text(s) => Box::new(Cursor::new(s.into_bytes())),
                };

                self.current = Some(read);
            } else {
                // No current part, and no parts left means there is nothing
                // left to write.
                //
                return Ok(Async::Ready(None));
            }
        }

        let num = if let Some(ref mut read) = self.current {
            // TODO: This should not write more bytes that are remaining in
            // the buffer.
            //
            io::copy(read, &mut writer)?
        } else {
            0
        };

        if num == 0 {
            // Wrote 0 bytes from the reader, so we reached the EOF for the
            // current item.
            //
            self.current = None;

            // Peek to check if there are are any parts not yet written.
            // If there is nothing, the final boundary can be written.
            //
            if self.parts.peek().is_none() {
                self.write_final_boundary(&mut writer)?;

                Ok(Async::Ready(Some(writer.into_inner())))
            } else {
                self.poll()
            }
        } else {
            Ok(Async::Ready(Some(writer.into_inner())))
        }
    }
}


/// Implements the multipart/form-data media type as described by
/// RFC 7578.
///
/// [See](https://tools.ietf.org/html/rfc7578#section-1).
///
pub struct Form {
    parts: Vec<Part>,

    /// The auto-generated boundary as described by 4.1.
    ///
    /// [See](https://tools.ietf.org/html/rfc7578#section-4.1).
    ///
    boundary: String,
}

impl Default for Form {
    /// Creates a new form with the default boundary generator.
    ///
    #[inline]
    fn default() -> Form {
        Form::new::<RandomAsciiGenerator>()
    }
}

impl Form {
    /// Creates a new form with the specified boundary generator function.
    ///
    #[inline]
    pub fn new<G>() -> Form
    where
        G: BoundaryGenerator,
    {
        Form {
            parts: vec![],
            boundary: G::generate_boundary(),
        }
    }

    /// Updates a request instance with the multipart Content-Type header
    /// and the payload data.
    ///
    pub fn set_body(self, req: &mut Request<Body>) {
        let header = format!("multipart/form-data; boundary=\"{}\"", &self.boundary);

        {
            let headers = req.headers_mut();

            headers.set(ContentType(Mime::from_str(&header).expect(
                "multipart mime type should parse",
            )));
        }

        req.set_body(self);
    }

    /// Adds a struct that implements Read.
    ///
    pub fn add_reader<F, R>(&mut self, name: F, read: R)
    where
        F: Into<String>,
        R: 'static + Read + Send,
    {
        let read = Box::new(read);

        self.parts.push(Part::new::<_, String>(
            Inner::Read(read, None),
            name,
            None,
            None,
        ));
    }

    /// Adds a file, and attempts to derive the mime type.
    ///
    #[inline]
    pub fn add_file<P, F>(&mut self, name: F, path: P) -> io::Result<()>
    where
        P: AsRef<Path>,
        F: Into<String>,
    {
        self.add_file_with_mime(name, path, None)
    }

    /// Adds a file with the specified mime type to the form.
    /// If the mime type isn't specified, a mime type will try to
    /// be derived.
    ///
    fn add_file_with_mime<P, F>(&mut self, name: F, path: P, mime: Option<Mime>) -> io::Result<()>
    where
        P: AsRef<Path>,
        F: Into<String>,
    {
        let f = File::open(&path)?;
        let mime = if let Some(ext) = path.as_ref().extension() {
            Mime::from_str(ext.to_string_lossy().borrow()).ok()
        } else {
            mime
        };
        let len = match f.metadata() {
            // If the path is not a file, it can't be uploaded because there
            // is no content.
            //
            Ok(ref meta) if !meta.is_file() => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "expected a file not directory",
            )),

            // If there is some metadata on the file, try to derive some
            // header values.
            //
            Ok(ref meta) => Ok(Some(meta.len())),

            // The file metadata could not be accessed. This MIGHT not be an
            // error, if the file could be opened.
            //
            Err(e) => Err(e),
        }?;

        let read = Box::new(f);

        self.parts.push(Part::new(
            Inner::Read(read, len),
            name,
            mime,
            Some(path.as_ref().as_os_str().to_string_lossy()),
        ));

        Ok(())
    }
}

impl Into<Body> for Form {
    #[inline]
    fn into(self) -> Body {
        Body {
            buf_size: 2048,
            current: None,
            parts: self.parts.into_iter().peekable(),
            boundary: self.boundary,
        }
    }
}


pub struct Part {
    inner: Inner,

    /// Each part can include a Content-Type header field. If this
    /// is not specified, it defaults to "text/plain", or
    /// "application/octet-stream" for file data.
    ///
    /// [See](https://tools.ietf.org/html/rfc7578#section-4.4)
    ///
    content_type: ContentType,

    /// Each part must contain a Content-Disposition header field.
    ///
    /// [See](https://tools.ietf.org/html/rfc7578#section-4.2).
    ///
    content_disposition: ContentDisposition,
}

impl Part {
    /// Internal method to build a new Part instance. Sets the disposition type,
    /// content-type, and the disposition parameters for name, and optionally
    /// for filename.
    ///
    /// Per [4.3](https://tools.ietf.org/html/rfc7578#section-4.3), if multiple
    /// files need to be specified for one form field, they can all be specified
    /// with the same name parameter.
    ///
    fn new<N, F>(inner: Inner, name: N, mime: Option<Mime>, filename: Option<F>) -> Part
    where
        N: Into<String>,
        F: Into<String>,
    {
        // `name` disposition parameter is required. It should correspond to the
        // name of a form field.
        //
        // [See 4.2](https://tools.ietf.org/html/rfc7578#section-4.2)
        //
        let mut disposition_params = vec![DispositionParam::Ext("name".into(), name.into())];

        // `filename` can be supplied for files, but is totally optional.
        //
        // [See 4.2](https://tools.ietf.org/html/rfc7578#section-4.2)
        //
        if let Some(filename) = filename {
            disposition_params.push(DispositionParam::Ext("filename".into(), filename.into()));
        }

        let content_type = ContentType(mime.unwrap_or_else(|| inner.default_content_type()));

        Part {
            inner: inner,
            content_type: content_type,
            content_disposition: ContentDisposition {
                disposition: DispositionType::Ext("form-data".into()),
                parameters: disposition_params,
            },
        }
    }
}


enum Inner {
    /// The `Read` variant captures multiple cases.
    ///
    ///   * The first is it supports uploading a file, which is explicitly
    ///     described in RFC 7578.
    ///
    ///   * The second (which is not described by RFC 7578), is it can handle
    ///     arbitrary input streams (for example, a server response).
    ///     Any arbitrary input stream is automatically considered a file,
    ///     and assigned the corresponding content type if not explicitly
    ///     specified.
    ///
    Read(Box<'static + Read + Send>, Option<u64>),

    /// The `String` variant handles "text/plain" form data payloads.
    ///
    Text(String),
}

impl Inner {
    /// Returns the default Content-Type header value as described in section 4.4.
    ///
    /// [See](https://tools.ietf.org/html/rfc7578#section-4.4)
    ///
    #[inline]
    fn default_content_type(&self) -> Mime {
        match self {
            &Inner::Read(_, _) => mime::APPLICATION_OCTET_STREAM,
            &Inner::Text(_) => mime::TEXT_PLAIN,
        }
    }

    /// Returns the length of the inner type.
    ///
    #[inline]
    fn len(&self) -> Option<u64> {
        match self {
            &Inner::Read(_, len) => len,
            &Inner::Text(ref s) => Some(s.len() as u64),
        }
    }
}


/// Random boundary string provider.
///
pub trait BoundaryGenerator {
    /// Generates a String to use as a boundary.
    ///
    fn generate_boundary() -> String;
}


struct RandomAsciiGenerator;

impl BoundaryGenerator for RandomAsciiGenerator {
    /// Creates a boundary of 6 ascii characters.
    ///
    fn generate_boundary() -> String {
        let mut rng = rand::weak_rng();
        let ascii = rng.gen_ascii_chars();

        String::from_iter(ascii.take(6))
    }
}
