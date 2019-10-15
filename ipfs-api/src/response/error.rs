// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::serde::Deserialize;

#[cfg(feature = "actix")]
use awc;
use http;
#[cfg(feature = "hyper")]
use hyper;
use serde_json;
use serde_urlencoded;
use std;
use std::string::FromUtf8Error;

#[cfg_attr(feature = "actix", derive(Display), display(fmt = "{}", message))]
#[cfg_attr(feature = "hyper", derive(Fail), fail(display = "{}", message))]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApiError {
    pub message: String,
    pub code: u8,
}

#[cfg_attr(feature = "actix", derive(Display))]
#[cfg_attr(feature = "hyper", derive(Fail))]
#[derive(Debug)]
pub enum Error {
    /// Foreign errors.
    #[cfg(feature = "hyper")]
    #[cfg_attr(feature = "hyper", fail(display = "hyper client error '{}'", _0))]
    Client(hyper::Error),

    #[cfg(feature = "actix")]
    #[cfg_attr(
        feature = "actix",
        display(fmt = "actix client payload error '{}'", _0)
    )]
    ClientPayload(awc::error::PayloadError),

    #[cfg(feature = "actix")]
    #[cfg_attr(
        feature = "actix",
        display(fmt = "actix client send request error '{}'", _0)
    )]
    ClientSend(awc::error::SendRequestError),

    #[cfg_attr(feature = "actix", display(fmt = "http error '{}'", _0))]
    #[cfg_attr(feature = "hyper", fail(display = "http error '{}'", _0))]
    Http(http::Error),

    #[cfg_attr(feature = "actix", display(fmt = "json parse error '{}'", _0))]
    #[cfg_attr(feature = "hyper", fail(display = "json parse error '{}'", _0))]
    Parse(serde_json::Error),

    #[cfg_attr(feature = "actix", display(fmt = "utf8 decoding error '{}'", _0))]
    #[cfg_attr(feature = "hyper", fail(display = "utf8 decoding error '{}'", _0))]
    ParseUtf8(FromUtf8Error),

    #[cfg_attr(feature = "actix", display(fmt = "uri error '{}'", _0))]
    #[cfg_attr(feature = "hyper", fail(display = "uri error '{}'", _0))]
    Url(http::uri::InvalidUri),

    #[cfg_attr(feature = "actix", display(fmt = "io error '{}'", _0))]
    #[cfg_attr(feature = "hyper", fail(display = "io error '{}'", _0))]
    Io(std::io::Error),

    #[cfg_attr(feature = "actix", display(fmt = "url encoding error '{}'", _0))]
    #[cfg_attr(feature = "hyper", fail(display = "url encoding error '{}'", _0))]
    EncodeUrl(serde_urlencoded::ser::Error),

    /// An error returned by the Ipfs api.
    #[cfg_attr(feature = "actix", display(fmt = "api returned error '{}'", _0))]
    #[cfg_attr(feature = "hyper", fail(display = "api returned error '{}'", _0))]
    Api(ApiError),

    /// A stream error indicated in the Trailer header.
    #[cfg_attr(
        feature = "actix",
        display(fmt = "api returned an error while streaming: '{}'", _0)
    )]
    #[cfg_attr(
        feature = "hyper",
        fail(display = "api returned an error while streaming: '{}'", _0)
    )]
    StreamError(String),

    /// API returned a trailer header with unrecognized value.
    #[cfg_attr(
        feature = "actix",
        display(fmt = "api returned a trailer header with unknown value: '{}'", _0)
    )]
    #[cfg_attr(
        feature = "hyper",
        fail(display = "api returned a trailer header with unknown value: '{}'", _0)
    )]
    UnrecognizedTrailerHeader(String),

    #[cfg_attr(
        feature = "actix",
        display(fmt = "api returned unknwon error '{}'", _0)
    )]
    #[cfg_attr(
        feature = "hyper",
        fail(display = "api returned unknwon error '{}'", _0)
    )]
    Uncategorized(String),
}

#[cfg(feature = "hyper")]
impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Client(err)
    }
}

#[cfg(feature = "actix")]
impl From<awc::error::SendRequestError> for Error {
    fn from(err: awc::error::SendRequestError) -> Error {
        Error::ClientSend(err)
    }
}

#[cfg(feature = "actix")]
impl From<awc::error::PayloadError> for Error {
    fn from(err: awc::error::PayloadError) -> Error {
        Error::ClientPayload(err)
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
