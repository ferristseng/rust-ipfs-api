// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use ipfs_api::IpfsClient;
use std::io::Cursor;

// Creates an Ipfs client, and adds this dag object to Ipfs then fetch it back.
//
#[cfg_attr(feature = "actix", actix_rt::main)]
#[cfg_attr(feature = "hyper", tokio::main)]
async fn main() {
    eprintln!("note: this must be run in the root of the project repository");
    eprintln!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    let dag_node = Cursor::new(r#"{ "hello" : "world" }"#);

    let response = client
        .dag_put(dag_node)
        .await
        .expect("error adding dag node");

    let cid = response.cid.cid_string;

    let response = client.dag_get(&cid).await.expect("error getting dag node");

    println!("dag node => {}", response);
}
