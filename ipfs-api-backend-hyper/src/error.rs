// Copyright 2021 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("api returned error `{0}`")]
    Api(ipfs_api_prelude::ApiError),

    #[error("hyper client error `{0}`")]
    Client(#[from] hyper::Error),

    #[error("http error `{0}`")]
    Http(#[from] http::Error),

    #[error("ipfs client error `{0}`")]
    IpfsClientError(#[from] ipfs_api_prelude::Error),
}

impl From<ipfs_api_prelude::ApiError> for Error {
    fn from(err: ipfs_api_prelude::ApiError) -> Self {
        Error::Api(err)
    }
}
