// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use ipfs_api::IpfsClient;

// Creates an Ipfs client, and gets information about your local address, and
// connected peers.
//
#[cfg_attr(feature = "with-actix", actix_rt::main)]
#[cfg_attr(any(feature = "with-hyper", feature = "with-reqwest"), tokio::main)]
async fn main() {
    tracing_subscriber::fmt::init();

    eprintln!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    match client.swarm_addrs_local().await {
        Ok(local) => {
            eprintln!("your addrs:");
            for addr in local.strings {
                eprintln!("  {}", addr);
            }
            eprintln!();
        }
        Err(e) => eprintln!("error getting local swarm addresses: {}", e),
    }

    match client.swarm_peers().await {
        Ok(connected) => {
            eprintln!("connected:");
            for peer in connected.peers {
                let streams: Vec<&str> = peer.streams.iter().map(|s| &s.protocol[..]).collect();
                eprintln!("  addr:     {}", peer.addr);
                eprintln!("  peer:     {}", peer.peer);
                eprintln!("  latency:  {}", peer.latency);
                eprintln!("  muxer:    {}", peer.muxer);
                eprintln!("  streams:  {}", streams.join(", "));
                eprintln!();
            }
        }
        Err(e) => eprintln!("error getting swarm peers: {}", e),
    }
}
