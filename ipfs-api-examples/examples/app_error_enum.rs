// Copyright 2022 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Demonstrates how to define an application error enum that has an ipfs_api::Error as possibility.

use ipfs_api_examples::ipfs_api;
use ipfs_api_examples::ipfs_api::response::VersionResponse;
use ipfs_api_examples::ipfs_api::{IpfsApi, IpfsClient};

/// An error type that is either an IPFS client error, or some other error.
#[derive(Debug, thiserror::Error)]
enum ApplicationError {
    #[error("IPFS error: {0}")]
    Ipfs(#[from] ipfs_api::Error),

    #[error("{0}")]
    Message(String),
}

/// Similar to get_version example, except implementation is in a function called by main rather
/// than in main itself, and the callee, use_client, returns Result with a custom error type.
#[ipfs_api_examples::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let client = IpfsClient::default();

    // Get and print version, using client directly
    if let Err(e) = use_api(&client).await {
        eprintln!("{:?}", e);
    }

    // Get and print version, using client by its trait IpfsApi
    if let Err(e) = use_client(&client).await {
        eprintln!("{:?}", e);
    }
}

fn print_version(version: VersionResponse) -> Result<(), ApplicationError> {
    if version.version.starts_with("0.0") {
        Err(ApplicationError::Message(format!(
            "Version {} is very old",
            version.version
        )))
    } else {
        println!("Version: {}", version.version);
        Ok(())
    }
}

async fn use_api<A: IpfsApi<Error = ipfs_api::Error>>(client: &A) -> Result<(), ApplicationError> {
    let version = client.version().await?;

    print_version(version)
}

async fn use_client(client: &IpfsClient) -> Result<(), ApplicationError> {
    let version = client.version().await?;

    print_version(version)
}
