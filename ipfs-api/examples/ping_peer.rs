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

// Creates an Ipfs client, discovers a connected peer, and pings it using the
// streaming Api, and by collecting it into a collection.
//
fn main() {
    println!("connecting to localhost:5001...");

    let mut core = Core::new().expect("expected event loop");
    let client = IpfsClient::default(&core.handle());

    println!();
    println!("discovering connected peers...");

    let connected = client.swarm_peers();
    let connected = core.run(connected).expect("expected a valid response");

    let peer = connected.peers.iter().next().expect(
        "expected at least one peer",
    );

    println!();
    println!("discovered peer ({})", peer.peer);
    println!();
    println!("streaming 10 pings...");
    let req = client.ping(&peer.peer[..], &Some(10));

    core.run(req.for_each(|ping| {
        println!("{:?}", ping);
        Ok(())
    })).expect("expected a valid response");

    println!();
    println!("gathering 15 pings...");

    let req = client.ping(&peer.peer[..], &Some(15));
    let pings: Vec<_> = core.run(req.collect()).expect("expected a valid response");

    for ping in pings.iter() {
        println!("got response ({:?}) in ({})...", ping.text, ping.time);
    }
}
