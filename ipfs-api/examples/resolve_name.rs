// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

extern crate futures;
extern crate ipfs_api;
extern crate tokio;

use futures::Future;
use ipfs_api::IpfsClient;
use tokio::runtime::current_thread::Runtime;

const IPFS_IPNS: &str = "/ipns/ipfs.io";

// Creates an Ipfs client, and resolves the Ipfs domain name, and
// publishes a path to Ipns.
//
fn main() {
    println!("connecting to localhost:5001...");

    let client = IpfsClient::default();
    let name_resolve = client
        .name_resolve(Some(IPFS_IPNS), true, false)
        .map(|resolved| {
            println!("{} resolves to: {}", IPFS_IPNS, &resolved.path);
        });

    let name_publish = client
        .name_publish(IPFS_IPNS, true, None, None, None)
        .and_then(move |publish| {
            println!("published {} to: /ipns/{}", IPFS_IPNS, &publish.name);

            client
                .name_resolve(Some(&publish.name), true, false)
                .map(move |resolved| {
                    println!("/ipns/{} resolves to: {}", &publish.name, &resolved.path);
                })
        });

    let fut = name_resolve
        .and_then(|_| name_publish)
        .map_err(|e| eprintln!("{}", e));

    Runtime::new()
        .expect("tokio runtime")
        .block_on(fut)
        .expect("successful response");
}
