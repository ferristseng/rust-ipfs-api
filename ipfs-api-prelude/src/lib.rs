// Copyright 2021 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

#[cfg(feature = "with-builder")]
#[macro_use]
extern crate typed_builder;

extern crate serde;

mod api;
mod backend;
mod error;
mod from_uri;
mod global_opts;
mod header;
mod read;
pub mod request;
pub mod response;

pub use {
    api::IpfsApi,
    backend::{Backend, BoxStream},
    error::Error,
    from_uri::TryFromUri,
    global_opts::{BackendWithGlobalOptions, GlobalOptions},
    request::ApiRequest,
    response::ApiError,
};
