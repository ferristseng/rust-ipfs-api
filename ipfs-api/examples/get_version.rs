// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use ipfs_api::IpfsClient;

// Creates an Ipfs client, and gets the version of the Ipfs server.
//
#[tokio::main]
async fn main() {
    eprintln!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    match client.version().await {
        Ok(version) => eprintln!("version: {:?}", version.version),
        Err(e) => eprintln!("error getting version: {}", e),
    }
}
