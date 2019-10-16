// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use futures::Future;
use ipfs_api::IpfsClient;
use std::fs::File;
use tokio::runtime::current_thread::Runtime;

// Creates an Ipfs client, and adds this source file to Ipfs.
//
fn main() {
    println!("note: this must be run in the root of the project repository");
    println!("connecting to localhost:5001...");

    let client = IpfsClient::default();
    let file = File::open(file!()).expect("could not read source file");
    let req = client
        .add(file)
        .map(|add| println!("added file: {:?}", add))
        .map_err(|e| eprintln!("{}", e));

    Runtime::new()
        .expect("tokio runtime")
        .block_on(req)
        .expect("successful response");
}
