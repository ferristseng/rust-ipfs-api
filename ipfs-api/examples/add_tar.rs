// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use futures::TryStreamExt;
use ipfs_api::IpfsClient;
use std::io::Cursor;
use tar::Builder;

// Creates an Ipfs client, and adds this source file to Ipfs.
//
#[cfg_attr(feature = "with-actix", actix_rt::main)]
#[cfg_attr(feature = "with-hyper", tokio::main)]
#[cfg_attr(feature = "with-reqwest", tokio::main)]
async fn main() {
    tracing_subscriber::fmt::init();

    eprintln!("note: this must be run in the root of the project repository");
    eprintln!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    // Create a in-memory tar file with this source file as its contents.
    //
    let mut buf = Vec::new();
    {
        let mut builder = Builder::new(&mut buf);

        builder
            .append_path(file!())
            .expect("failed to create tar file");
        builder.finish().expect("failed to create tar file");
    }
    let cursor = Cursor::new(buf);

    // Write in-memory tar file to IPFS.
    //
    let file = match client.tar_add(cursor).await {
        Ok(file) => {
            eprintln!("added tar file: {:?}", file);
            eprintln!();

            file
        }
        Err(e) => {
            eprintln!("error writing tar file: {}", e);

            return;
        }
    };

    // Read tar file from IPFS.
    //
    match client
        .tar_cat(&file.hash[..])
        .map_ok(|chunk| chunk.to_vec())
        .try_concat()
        .await
    {
        Ok(tar) => {
            println!("{}", String::from_utf8_lossy(&tar[..]));
        }
        Err(e) => {
            eprintln!("error reading tar file: {}", e);
        }
    }
}
