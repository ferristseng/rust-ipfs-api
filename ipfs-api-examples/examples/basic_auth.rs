// Copyright 2022 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::env;

use ipfs_api_examples::ipfs_api::{IpfsApi, IpfsClient, TryFromUri};

fn expect_env(key: &str) -> String {
    env::var(key).expect(key)
}

/// Creates an IPFS client, and sets credentials to use.
#[ipfs_api_examples::main]
async fn main() {
    let ipfs_api_url = expect_env("IPFS_API_URL");
    let ipfs_api_username = expect_env("IPFS_API_USERNAME");
    let ipfs_api_password = expect_env("IPFS_API_PASSWORD");

    println!("Connecting to {} as {}...", ipfs_api_url, ipfs_api_username);

    let client = IpfsClient::from_str(&ipfs_api_url)
        .unwrap()
        .with_credentials(ipfs_api_username, ipfs_api_password);

    match client.version().await {
        Ok(version) => println!("version: {}", version.version),
        Err(e) => eprintln!("error getting version: {}", e),
    }
}
