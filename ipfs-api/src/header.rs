// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use hyper;
use hyper::header::{self, Header, Raw};
use std::fmt;

/// Header that is returned for streaming calls.
///
/// A `Trailer` header indicates that after a streaming call, there will
/// be some additional information in the response.
///
#[derive(Debug, Clone, Copy)]
pub enum Trailer {
    /// This trailer indicates that an error header will be returned in
    /// the stream if there is an error while streaming.
    ///
    StreamError,
}

impl Header for Trailer {
    fn header_name() -> &'static str {
        "Trailer"
    }

    fn parse_header(raw: &Raw) -> hyper::Result<Trailer> {
        if let Some(bytes) = raw.one() {
            let value = String::from_utf8_lossy(bytes);

            match value.as_ref() {
                "X-Stream-Error" => Ok(Trailer::StreamError),
                _ => Err(hyper::Error::Header),
            }
        } else {
            Err(hyper::Error::Header)
        }
    }

    fn fmt_header(&self, f: &mut header::Formatter) -> fmt::Result {
        let value = match *self {
            Trailer::StreamError => "X-Stream-Error",
        };

        f.fmt_line(&value)
    }
}

/// This header is included while streaming if an error occured
/// while streaming the data.
///
#[derive(Debug, Clone)]
pub struct XStreamError {
    pub error: String,
}

impl Header for XStreamError {
    fn header_name() -> &'static str {
        "X-Stream-Error"
    }

    fn parse_header(raw: &Raw) -> hyper::Result<XStreamError> {
        if let Some(bytes) = raw.one() {
            let value = String::from_utf8_lossy(bytes);

            Ok(XStreamError {
                error: value.into_owned(),
            })
        } else {
            Err(hyper::Error::Header)
        }
    }

    fn fmt_header(&self, f: &mut header::Formatter) -> fmt::Result {
        f.fmt_line(&self.error)
    }
}
