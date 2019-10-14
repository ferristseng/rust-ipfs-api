// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

extern crate futures;
extern crate ipfs_api;
extern crate tokio;

use futures::Future;
use ipfs_api::IpfsClient;
use tokio::runtime::current_thread::Runtime;

// Creates an Ipfs client, and gets the version of the Ipfs server.
//
fn main() {
    println!("connecting to localhost:5001...");

    let client = IpfsClient::default();
    let req = client
        .version()
        .map(|version| println!("version: {:?}", version.version));

    let fut = req.map_err(|e| eprintln!("{}", e));

    Runtime::new()
        .expect("tokio runtime")
        .block_on(fut)
        .expect("successful response");
}
