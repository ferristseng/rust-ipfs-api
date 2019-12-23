// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use ipfs_api::IpfsClient;

// Lists clients in bootstrap list, then adds the default list, then removes
// them, and readds them.
//
#[tokio::main]
async fn main() {
    eprintln!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    match client.bootstrap_list().await {
        Ok(bootstrap) => {
            eprintln!("current bootstrap peers:");
            for peer in bootstrap.peers {
                eprintln!("  {}", peer);
            }
            eprintln!();
        }
        Err(e) => {
            eprintln!("error getting list of bootstrap peers: {}", e);
            return;
        }
    }

    match client.bootstrap_rm_all().await {
        Ok(drop) => {
            eprintln!("dropped:");
            for peer in drop.peers {
                eprintln!("  {}", peer);
            }
            eprintln!();
        }
        Err(e) => {
            eprintln!("error dropping bootstrap peers: {}", e);
        }
    }

    match client.bootstrap_add_default().await {
        Ok(add) => {
            eprintln!("added:");
            for peer in add.peers {
                eprintln!("  {}", peer);
            }
            eprintln!();
        }
        Err(e) => {
            eprintln!("error adding default peers: {}", e);
        }
    }
}
