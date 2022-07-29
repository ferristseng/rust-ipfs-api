// Copyright 2022 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use futures::TryStreamExt;
use ipfs_api_examples::ipfs_api::{IpfsApi, IpfsClient};
use std::fs::File;

// Creates an Ipfs client, uploads a file, and reads it.
//
#[ipfs_api_examples::main]
async fn main() -> Result<(), &'static str> {
    tracing_subscriber::fmt::init();

    eprintln!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    eprintln!("adding this example source file to IPFS...");

    let file = File::open(file!()).expect("could not read source file");
    let added_file = client
        .add(file)
        .await
        .map_err(|_| "error adding source file")?;

    let offset = 1065;
    let length = 390;

    eprintln!("cat {} bytes from {}...", length, offset);

    let section = client
        .cat_range(&added_file.hash[..], offset, length)
        .map_ok(|chunk| chunk.to_vec())
        .try_concat()
        .await
        .map_err(|_| "error reading file from offset")?;
    eprintln!("---------------------------------------");
    eprintln!("{}", String::from_utf8_lossy(&section[..]));
    eprintln!("---------------------------------------");

    eprintln!("reading full file...");

    let all = client
        .cat(&added_file.hash[..])
        .map_ok(|chunk| chunk.to_vec())
        .try_concat()
        .await
        .map_err(|_| "error reading full file")?;
    eprintln!("---------------------------------------");
    eprintln!("{}", String::from_utf8_lossy(&all[..]));
    eprintln!("---------------------------------------");

    Ok(())
}
