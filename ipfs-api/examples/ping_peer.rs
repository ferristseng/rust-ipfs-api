// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

#[cfg(feature = "actix")]
extern crate actix_web;
extern crate futures;
#[cfg(feature = "hyper")]
extern crate hyper;
extern crate ipfs_api;

use futures::{Future, Stream};
use ipfs_api::{response::PingResponse, IpfsClient};

// Creates an Ipfs client, discovers a connected peer, and pings it using the
// streaming Api, and by collecting it into a collection.
//
fn main() {
    println!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    println!();
    println!("discovering connected peers...");

    let req = client
        .swarm_peers()
        .and_then(move |connected| {
            let peer = connected
                .peers
                .iter()
                .next()
                .expect("expected at least one peer");

            println!();
            println!("discovered peer ({})", peer.peer);
            println!();
            println!("streaming 10 pings...");

            let ping_stream = client.ping(&peer.peer[..], Some(10)).for_each(|ping| {
                println!("{:?}", ping);
                Ok(())
            });

            let ping_gather = client.ping(&peer.peer[..], Some(15)).collect();

            ping_stream.and_then(|_| {
                println!();
                println!("gathering 15 pings...");

                ping_gather
            })
        })
        .map(|pings: Vec<PingResponse>| {
            for ping in pings.iter() {
                println!("got response ({:?}) at ({})...", ping.text, ping.time);
            }
        })
        .map_err(|e| eprintln!("{}", e));

   #[cfg(feature = "hyper")]
    hyper::rt::run(req);
    #[cfg(feature = "actix")]
    actix_web::actix::run(|| {
        req.then(|_| {
            actix_web::actix::System::current().stop();
            Ok(())
        })
    });
}
