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

// Creates an Ipfs client, and adds this dag object to Ipfs then fetch it back.
//
#[cfg_attr(feature = "with-actix", actix_rt::main)]
#[cfg_attr(any(feature = "with-hyper", feature = "with-reqwest"), tokio::main)]
async fn main() {
    tracing_subscriber::fmt::init();

    eprintln!("note: this must be run in the root of the project repository");
    eprintln!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    let dag_node = Cursor::new(r#"{"hello":"world"}"#);

    let response = client
        .dag_put(dag_node)
        .await
        .expect("error adding dag node");

    let cid = response.cid.cid_string;

    match client
        .dag_get(&cid)
        .map_ok(|chunk| chunk.to_vec())
        .try_concat()
        .await
    {
        Ok(bytes) => {
            println!("{}", String::from_utf8_lossy(&bytes[..]));
        }
        Err(e) => {
            eprintln!("error reading dag node: {}", e);
        }
    }
}
