// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use futures::Future;
use ipfs_api::IpfsClient;
use tokio::runtime::current_thread::Runtime;

// Creates an Ipfs client, and gets information about your local address, and
// connected peers.
//
fn main() {
    println!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    let local = client.swarm_addrs_local().map(|local| {
        println!();
        println!("your addrs:");
        for addr in local.strings {
            println!("  {}", addr);
        }
    });

    let connected = client.swarm_peers().map(|connected| {
        println!();
        println!("connected:");
        for peer in connected.peers {
            let streams: Vec<&str> = peer.streams.iter().map(|s| &s.protocol[..]).collect();
            println!("  addr:     {}", peer.addr);
            println!("  peer:     {}", peer.peer);
            println!("  latency:  {}", peer.latency);
            println!("  muxer:    {}", peer.muxer);
            println!("  streams:  {}", streams.join(", "));
            println!();
        }
    });

    let fut = local
        .and_then(|_| connected)
        .map_err(|e| eprintln!("{}", e));

    Runtime::new()
        .expect("tokio runtime")
        .block_on(fut)
        .expect("successful response");
}
