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

// Lists clients in bootstrap list, then adds the default list, then removes
// them, and readds them.
//
fn main() {
    println!("connecting to localhost:5001...");

    let mut core = Core::new().expect("expected event loop");
    let client = IpfsClient::default(&core.handle());

    let bootstrap = client.bootstrap_list();
    let bootstrap = core.run(bootstrap).expect("expected a valid response");

    println!("current bootstrap peers:");
    for peer in bootstrap.peers {
        println!("  {}", peer);
    }

    println!();
    println!("dropping all bootstrap peers...");

    let drop = client.bootstrap_rm_all();
    let drop = core.run(drop).expect("expected a valid response");

    println!("dropped:");
    for peer in drop.peers {
        println!("  {}", peer);
    }

    println!();
    println!("adding default peers...");

    let add = client.bootstrap_add_default();
    let add = core.run(add).expect("expected a valid response");

    println!("added:");
    for peer in add.peers {
        println!("  {}", peer);
    }
}
