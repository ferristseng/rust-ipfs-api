// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

extern crate futures;
extern crate hyper;
extern crate ipfs_api;

use futures::Future;
use ipfs_api::IpfsClient;

// Lists clients in bootstrap list, then adds the default list, then removes
// them, and readds them.
//
fn main() {
    println!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    let bootstrap = client.bootstrap_list().map(|bootstrap| {
        println!("current bootstrap peers:");
        for peer in bootstrap.peers {
            println!("  {}", peer);
        }
    });

    let drop = client.bootstrap_rm_all().map(|drop| {
        println!("dropped:");
        for peer in drop.peers {
            println!("  {}", peer);
        }
    });

    let add = client.bootstrap_add_default().map(|add| {
        println!("added:");
        for peer in add.peers {
            println!("  {}", peer);
        }
    });

    hyper::rt::run(
        bootstrap
            .and_then(|_| {
                println!();
                println!("dropping all bootstrap peers...");

                drop
            })
            .and_then(|_| {
                println!();
                println!("adding default peers...");

                add
            })
            .map_err(|e| eprintln!("{}", e)),
    );
}
