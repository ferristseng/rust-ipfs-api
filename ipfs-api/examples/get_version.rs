// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

extern crate futures;
extern crate ipfs_api;
extern crate tokio_core;

use futures::stream::Stream;
use ipfs_api::IpfsClient;
use tokio_core::reactor::Core;

// Creates an Ipfs client, and gets the version of the Ipfs server.
//
fn main() {
    println!("connecting to localhost:5001...");

    let mut core = Core::new().expect("expected event loop");
    let client = IpfsClient::default(&core.handle());
    let req = client.version();
    let version = core.run(req).expect("expected a valid response");

    println!("version: {:?}", version.version);

    let req = client.refs_local();
    println!(
        "{:?}",
        core.run(req.for_each(|res| Ok(println!("{:?}", res))))
    );

    let req = client.diag_sys();
    println!("{}", core.run(req).expect("response"));

    let req = client.dns("ipfs.io", false);
    let dns = core.run(req).expect("response");
    println!("{:?}", dns);

    let req = client.file_ls(&dns.path[..]);
    println!("{:?}", core.run(req).expect("response"));

    /*
    let req = client.dht_get("QmRJijhiMxQgn7bFP4cBsarHsGMM8g9fLDEE3WtkTXr4Hr");
    println!(
        "{:?}",
        core.run(req.for_each(|res| Ok(println!("{:?}", res))))
    );
    */
}
