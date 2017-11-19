// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use hyper::header::{ContentDisposition, ContentType, DispositionParam, DispositionType};
use hyper::mime::{self, Mime};
use rand::{self, Rng};
use std::borrow::{Borrow, Cow};
use std::fs::File;
use std::io::{self, Read};
use std::iter::FromIterator;
use std::path::Path;
use std::str::FromStr;


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

    /// Implements section 4.1.
    ///
    /// [See](https://tools.ietf.org/html/rfc7578#section-4.1).
    ///
    fn write_boundary(&self) -> io::Result<()> {
        Ok(())
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
            Ok(ref meta) => Ok(meta.len()),

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


pub struct Part {
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

        Part {
            content_type: ContentType(mime.unwrap_or_else(|| inner.default_content_type())),
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
    Read(Box<Read>, u64),

    /// The `String` variant handles "text/plain" form data payloads.
    ///
    String(Cow<'static, str>),
}

impl Inner {
    /// Returns the default Content-Type header value as described in section 4.4.
    ///
    /// [See](https://tools.ietf.org/html/rfc7578#section-4.4)
    ///
    #[inline]
    fn default_content_type(&self) -> Mime {
        match self {
            &Inner::Read(_, _) => mime::TEXT_PLAIN,
            &Inner::String(_) => mime::APPLICATION_OCTET_STREAM,
        }
    }

    /// Returns the length of the inner type.
    ///
    #[inline]
    fn len(&self) -> u64 {
        match self {
            &Inner::Read(_, len) => len,
            &Inner::String(ref s) => s.len() as u64,
        }
    }
}
