// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use ipfs_api::IpfsClient;

const IPFS_IPNS: &str = "/ipns/ipfs.io";

// Creates an Ipfs client, and resolves the Ipfs domain name, and
// publishes a path to Ipns.
//
#[tokio::main]
async fn main() {
    eprintln!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    match client.name_resolve(Some(IPFS_IPNS), true, false).await {
        Ok(resolved) => eprintln!("{} resolves to: {}", IPFS_IPNS, &resolved.path),
        Err(e) => {
            eprintln!("error resolving {}: {}", IPFS_IPNS, e);
            return;
        }
    }

    let publish = match client.name_publish(IPFS_IPNS, true, None, None, None).await {
        Ok(publish) => {
            eprintln!("published {} to: /ipns/{}", IPFS_IPNS, &publish.name);
            publish
        }
        Err(e) => {
            eprintln!("error publishing name: {}", e);
            return;
        }
    };

    match client.name_resolve(Some(&publish.name), true, false).await {
        Ok(resolved) => {
            eprintln!("/ipns/{} resolves to: {}", &publish.name, &resolved.path);
        }
        Err(e) => {
            eprintln!("error resolving name: {}", e);
        }
    }
}
