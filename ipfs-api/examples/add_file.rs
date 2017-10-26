// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

extern crate ipfs_api;
extern crate tokio_core;

use ipfs_api::IpfsClient;
use std::fs::File;
use tokio_core::reactor::Core;

// Creates an Ipfs client, and adds this source file to Ipfs.
//
fn main() {
    println!("note: this must be run in the root of the project repository");
    println!("connecting to localhost:5001...");

    let mut core = Core::new().expect("expected event loop");
    let client = IpfsClient::default(&core.handle());
    let file = File::open(file!()).expect("could not read source file");
    let req = client.add(file);
    let add = core.run(req).expect("expected a valid response");

    println!("added file: {:?}", add);
}
