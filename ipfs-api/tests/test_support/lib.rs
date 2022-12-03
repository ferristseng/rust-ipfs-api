// Copyright 2022 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// Each test compiles separately, but not all tests use everything in test support.
#![allow(dead_code)]

mod resources;

pub mod client;
pub mod container;
pub mod errors;
pub mod images;
pub mod rt;
