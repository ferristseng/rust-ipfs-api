// Copyright 2022 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::future::Future;

/// Hyper tests can use [tokio::test] but Actix can't due to LocalSet requirement.
pub fn run_async<F: Future>(f: F) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Actix requires LocalSet. Hyper doesn't care.
    let local = tokio::task::LocalSet::new();
    local.block_on(&rt, f);
}
