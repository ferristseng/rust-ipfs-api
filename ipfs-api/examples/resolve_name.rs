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

const IPFS_IPNS: &str = "/ipns/ipfs.io";

// Creates an Ipfs client, and resolves the Ipfs domain name, and
// publishes a path to Ipns.
//
fn main() {
    println!("connecting to localhost:5001...");

    let mut core = Core::new().expect("expected event loop");
    let client = IpfsClient::default(&core.handle());
    let req = client.name_resolve(Some(IPFS_IPNS), true, false);
    let resolved = core.run(req).expect("expected a valid response");

    println!("{} resolves to: {}", IPFS_IPNS, &resolved.path);

    let req = client.name_publish(IPFS_IPNS, true, None, None, None);
    let publish = core.run(req).expect("expected a valid response");

    println!("published {} to: /ipns/{}", IPFS_IPNS, &publish.name);

    let req = client.name_resolve(Some(&publish.name), true, false);
    let resolved = core.run(req).expect("expected a valid response");

    println!("/ipns/{} resolves to: {}", &publish.name, &resolved.path);
}
