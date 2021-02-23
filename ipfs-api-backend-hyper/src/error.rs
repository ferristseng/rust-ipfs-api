// Copyright 2019 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("api returned error `{0}`")]
    Api(ipfs_api_prelude::ApiError),

    #[error("hyper client error `{0}`")]
    Client(#[from] hyper::Error),

    #[error("http error `{0}`")]
    Http(#[from] http::Error),

    #[error("json parse error `{0}`")]
    Parse(#[from] serde_json::Error),

    #[error("utf8 decoding error `{0}`")]
    ParseUtf8(#[from] FromUtf8Error),

    #[error("uri error `{0}`")]
    Url(#[from] http::uri::InvalidUri),

    #[error("url encoding error `{0}`")]
    EncodeUrl(#[from] serde_urlencoded::ser::Error),

    #[error("ipfs client error `{0}`")]
    IpfsClientError(#[from] ipfs_api_prelude::Error),
}

impl From<ipfs_api_prelude::ApiError> for Error {
    fn from(err: ipfs_api_prelude::ApiError) -> Self {
        Error::Api(err)
    }
}
