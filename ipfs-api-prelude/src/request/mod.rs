// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

pub use self::add::*;
pub use self::bitswap::*;
pub use self::block::*;
pub use self::bootstrap::*;
pub use self::cat::*;
pub use self::commands::*;
pub use self::config::*;
pub use self::dag::*;
pub use self::dht::*;
pub use self::diag::*;
pub use self::dns::*;
pub use self::file::*;
pub use self::files::*;
pub use self::filestore::*;
pub use self::get::*;
pub use self::id::*;
pub use self::key::*;
pub use self::log::*;
pub use self::ls::*;
pub use self::name::*;
pub use self::object::*;
pub use self::pin::*;
pub use self::ping::*;
pub use self::pubsub::*;
pub use self::refs::*;
pub use self::shutdown::*;
pub use self::stats::*;
pub use self::swarm::*;
pub use self::swarm_connect::*;
pub use self::tar::*;
pub use self::version::*;

/// Create a test to verify that serializing a `ApiRequest` returns the expected
/// url encoded string.
///
#[cfg(test)]
macro_rules! serialize_url_test {
    ($f: ident, $obj: expr, $exp: expr) => {
        #[test]
        fn $f() {
            assert_eq!(::serde_urlencoded::to_string($obj), Ok($exp.to_string()))
        }
    };
}

/// Implements the `Serialize` trait for types that do not require
/// serialization. This provides a workaround for a limitation in
/// `serde_urlencoded`, that prevents unit structs from being serialized.
///
macro_rules! impl_skip_serialize {
    ($ty: ty) => {
        impl ::serde::Serialize for $ty {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                serializer.serialize_none()
            }
        }
    };
}

mod add;
mod bitswap;
mod block;
mod bootstrap;
mod cat;
mod commands;
mod config;
mod dag;
mod dht;
mod diag;
mod dns;
mod file;
mod files;
mod filestore;
mod get;
mod id;
mod key;
mod log;
mod ls;
mod name;
mod object;
mod pin;
mod ping;
mod pubsub;
mod refs;
mod shutdown;
mod stats;
mod swarm;
mod swarm_connect;
mod tar;
mod version;

use http::uri::Uri;
use serde::Serialize;

/// A request that can be made against the Ipfs API.
///
pub trait ApiRequest: Serialize + Send {
    /// Returns the API path that this request can be called on.
    ///
    /// All paths should begin with '/'.
    ///
    const PATH: &'static str;

    /// Method used to make the request.
    ///
    const METHOD: http::Method = http::Method::POST;

    /// Creates the absolute URL for an API resource given the base path
    /// of the service.
    ///
    fn absolute_url(&self, base: &Uri) -> Result<Uri, crate::Error> {
        format!(
            "{}{}?{}",
            base,
            Self::PATH,
            serde_urlencoded::to_string(self)?
        )
        .parse()
        .map_err(crate::Error::from)
    }
}
