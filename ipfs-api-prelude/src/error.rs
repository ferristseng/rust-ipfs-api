// Copyright 2021 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use std::{io, string::FromUtf8Error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error `{0}`")]
    Io(#[from] io::Error),

    #[error("utf8 decoding error `{0}`")]
    ParseUtf8(#[from] FromUtf8Error),

    #[error("json decoding error `{0}`")]
    Parse(#[from] serde_json::Error),

    #[error("uri error `{0}`")]
    Url(#[from] http::uri::InvalidUri),

    #[error("url encoding error `{0}`")]
    EncodeUrl(#[from] serde_urlencoded::ser::Error),

    #[error("api returned an error while streaming `{0}`")]
    StreamError(String),

    #[error("api got unrecognized trailer header `{0}`")]
    UnrecognizedTrailerHeader(String),

    #[error("api returned an unknown error `{0}`")]
    UnrecognizedApiError(String),
}
