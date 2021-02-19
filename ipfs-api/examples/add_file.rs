// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use ipfs_api::IpfsClient;

#[cfg(feature = "with-actix")]
#[cfg(feature = "with-hyper")]
use std::fs::File;

#[cfg(feature = "with-reqwest")]
use tokio::fs::File;

// Creates an Ipfs client, and adds this source file to Ipfs.
//
#[cfg_attr(feature = "with-actix", actix_rt::main)]
#[cfg_attr(features = "with-hyper", tokio::main)]
#[cfg_attr(feature = "with-reqwest", tokio::main)]
async fn main() {
    add_file().await
}

#[cfg(feature = "with-actix")]
#[cfg(feature = "with-hyper")]
async fn add_file() {
    tracing_subscriber::fmt::init();

    eprintln!("note: this must be run in the root of the project repository");
    eprintln!("connecting to localhost:5001...");

    let client = IpfsClient::default();
    let file = File::open(file!()).expect("could not read source file");

    match client.add(file).await {
        Ok(file) => eprintln!("added file: {:?}", file),
        Err(e) => eprintln!("error adding file: {}", e),
    }
}

#[cfg(feature = "with-reqwest")]
async fn add_file() {
    tracing_subscriber::fmt::init();

    eprintln!("note: this must be run in the root of the project repository");
    eprintln!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    let file = File::open(file!()).await.expect("Could open source file");

    match client.add(file).await {
        Ok(file) => eprintln!("added file: {:?}", file),
        Err(e) => eprintln!("error adding file: {}", e),
    }
}
