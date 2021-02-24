// Copyright 2019 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

extern crate hyper_multipart_rfc7578 as multipart;

mod backend;
mod error;

pub use crate::{backend::HyperBackend as IpfsClient, error::Error};
pub use ipfs_api_prelude::{
    request::{self, KeyType, Logger, LoggingLevel, ObjectTemplate},
    response, IpfsApi, TryFromUri,
};
