// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use ipfs_api_examples::ipfs_api::{IpfsApi, IpfsClient};

// Creates an Ipfs client, and gets the version of the Ipfs server.
//
#[ipfs_api_examples::main]
async fn main() {
    tracing_subscriber::fmt::init();

    eprintln!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    match client.version().await {
        Ok(version) => eprintln!("version: {:?}", version.version),
        Err(e) => eprintln!("error getting version: {}", e),
    }
}
