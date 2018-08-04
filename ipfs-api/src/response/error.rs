// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use http;
use hyper;
use serde_json;
use serde_urlencoded;
use std;
use std::string::FromUtf8Error;

#[derive(Fail, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[fail(display = "{}", message)]
pub struct ApiError {
    pub message: String,
    pub code: u8,
}

#[derive(Fail, Debug)]
pub enum Error {
    // Foreign errors.
    #[fail(display = "hyper client error '{}'", _0)]
    Client(hyper::Error),

    #[fail(display = "http error '{}'", _0)]
    Http(http::Error),

    #[fail(display = "json parse error '{}'", _0)]
    Parse(serde_json::Error),

    #[fail(display = "utf8 decoding error '{}'", _0)]
    ParseUtf8(FromUtf8Error),

    #[fail(display = "uri error '{}'", _0)]
    Url(http::uri::InvalidUri),

    #[fail(display = "io error '{}'", _0)]
    Io(std::io::Error),

    #[fail(display = "url encoding error '{}'", _0)]
    EncodeUrl(serde_urlencoded::ser::Error),

    /// An error returned by the Ipfs api.
    #[fail(display = "api returned error '{}'", _0)]
    Api(ApiError),

    /// A stream error indicated in the Trailer header.
    #[fail(display = "api returned an error while streaming: '{}'", _0)]
    StreamError(String),

    /// API returned a trailer header with unrecognized value.
    #[fail(
        display = "api returned a trailer header with unknown value: '{}'",
        _0
    )]
    UnrecognizedTrailerHeader(String),

    #[fail(display = "api returned unknwon error '{}'", _0)]
    Uncategorized(String),
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Client(err)
    }
}

impl From<http::Error> for Error {
    fn from(err: http::Error) -> Error {
        Error::Http(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Parse(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::ParseUtf8(err)
    }
}

impl From<http::uri::InvalidUri> for Error {
    fn from(err: http::uri::InvalidUri) -> Error {
        Error::Url(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_urlencoded::ser::Error> for Error {
    fn from(err: serde_urlencoded::ser::Error) -> Error {
        Error::EncodeUrl(err)
    }
}
