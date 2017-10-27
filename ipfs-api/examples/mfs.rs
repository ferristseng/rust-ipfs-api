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
use tokio_core::reactor::Core;

// Creates an Ipfs client, and makes some calls to the Mfs Api.
//
fn main() {
    println!("connecting to localhost:5001...");

    let mut core = Core::new().expect("expected event loop");
    let client = IpfsClient::default(&core.handle());

    println!("making /test...");
    println!("");

    let req = client.files_mkdir("/test", false);
    core.run(req).expect("expected mkdir to succeed");

    println!("making dirs /test/does/not/exist/yet...");
    println!("");

    let req = client.files_mkdir("/test/does/not/exist/yet", true);
    core.run(req).expect("expected mkdir -p to succeed");

    println!("getting status of /test/does...");
    println!("");

    let req = client.files_stat("/test/does");
    let stat = core.run(req).expect("expected stat to succeed");

    println!("  type     : {}", stat.typ);
    println!("  hash     : {}", stat.hash);
    println!("  size     : {}", stat.size);
    println!("  cum. size: {}", stat.cumulative_size);
    println!("  blocks   : {}", stat.blocks);
    println!("");

    println!("removing /test...");
    println!("");

    let req = client.files_rm("/test", true);
    core.run(req).expect("expected rm to succeed");

    println!("done!");
}
