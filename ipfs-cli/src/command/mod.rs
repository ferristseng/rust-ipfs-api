// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use std::error::Error;
use std::fs;


pub const EXPECTED_API: &'static str = "expected response from API";
pub const EXPECTED_FILE: &'static str = "expected to read input file";


/// Verifies that a path points to a file that exists, and not a directory.
///
pub fn verify_file(path: String) -> Result<(), String> {
    match fs::metadata(path) {
        Ok(ref metadata) if metadata.is_file() => Ok(()),
        Ok(_) => Err("file must not be a directory".into()),
        Err(e) => Err(e.description().into()),
    }
}


pub mod add;
pub mod bitswap;
pub mod block;
pub mod bootstrap;
pub mod cat;
pub mod commands;
pub mod config;
pub mod dag;
pub mod dht;
pub mod diag;
pub mod dns;
pub mod file;
pub mod files;
pub mod filestore;
pub mod version;
