// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

//! This module contains structures returned by the IPFS API.

use crate::serde::Deserialize;

pub use self::add::*;
pub use self::bitswap::*;
pub use self::block::*;
pub use self::bootstrap::*;
pub use self::commands::*;
pub use self::config::*;
pub use self::dag::*;
pub use self::dht::*;
pub use self::diag::*;
pub use self::dns::*;
pub use self::error::*;
pub use self::file::*;
pub use self::files::*;
pub use self::filestore::*;
pub use self::id::*;
pub use self::key::*;
pub use self::log::*;
pub use self::ls::*;
pub use self::mount::*;
pub use self::name::*;
pub use self::object::*;
pub use self::pin::*;
pub use self::ping::*;
pub use self::pubsub::*;
pub use self::refs::*;
pub use self::repo::*;
pub use self::resolve::*;
pub use self::shutdown::*;
pub use self::stats::*;
pub use self::swarm::*;
pub use self::swarm_connect::*;
pub use self::tar::*;
pub use self::version::*;

/// Create a test to deserialize a file to the given instance.
///
#[cfg(test)]
macro_rules! deserialize_test {
    ($f: ident, $ty: ident) => {
        #[test]
        fn $f() {
            let raw = include_str!(concat!("tests/", stringify!($f), ".json"));

            match ::serde_json::from_str::<super::$ty>(raw) {
                Ok(_) => assert!(true),
                Err(e) => assert!(false, "failed with error: {}", e),
            };
        }
    };
}

mod add;
mod bitswap;
mod block;
mod bootstrap;
mod commands;
mod config;
mod dag;
mod dht;
mod diag;
mod dns;
mod error;
mod file;
mod files;
mod filestore;
mod id;
mod key;
mod log;
mod ls;
mod mount;
mod name;
mod object;
mod pin;
mod ping;
mod pubsub;
mod refs;
mod repo;
mod resolve;
mod serde;
mod shutdown;
mod stats;
mod swarm;
mod swarm_connect;
mod tar;
mod version;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IpfsHeader {
    pub name: String,
    pub hash: String,
    pub size: u64,

    #[serde(rename = "Type")]
    pub typ: Option<String>,
}
