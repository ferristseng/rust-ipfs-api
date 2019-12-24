// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use futures::{future, TryStreamExt};
use ipfs_api::{response::PingResponse, IpfsClient};

// Creates an Ipfs client, discovers a connected peer, and pings it using the
// streaming Api, and by collecting it into a collection.
//
#[tokio::main]
async fn main() {
    eprintln!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    eprintln!();
    eprintln!("discovering connected peers...");

    let peer = match client.swarm_peers().await {
        Ok(connected) => connected
            .peers
            .into_iter()
            .next()
            .expect("expected at least one peer"),
        Err(e) => {
            eprintln!("error getting connected peers: {}", e);
            return;
        }
    };

    eprintln!();
    eprintln!("discovered peer ({})", peer.peer);
    eprintln!();
    eprintln!("streaming 10 pings...");

    if let Err(e) = client
        .ping(&peer.peer[..], Some(10))
        .try_for_each(|ping| {
            eprintln!("{:?}", ping);

            future::ok(())
        })
        .await
    {
        eprintln!("error streaming pings: {}", e);
    }

    eprintln!();
    eprintln!("gathering 15 pings...");

    match client
        .ping(&peer.peer[..], Some(15))
        .try_collect::<Vec<PingResponse>>()
        .await
    {
        Ok(pings) => {
            for ping in pings.iter() {
                eprintln!("got response ({:?}) at ({})...", ping.text, ping.time);
            }
        }
        Err(e) => eprintln!("error collecting pings: {}", e),
    }
}
