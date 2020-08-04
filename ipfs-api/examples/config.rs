// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use ipfs_api::IpfsClient;

// Creates an Ipfs client, read & set config values.
//
#[cfg_attr(feature = "actix", actix_rt::main)]
#[cfg_attr(feature = "hyper", tokio::main)]
async fn main() {
    eprintln!("note: this must be run in the root of the project repository");
    eprintln!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    //read a value
    let response = client
        .config("Identity.PeerID", None, None, None)
        .await
        .expect("Config read failed");

    println!("Config: {}={}", response.key, response.value);

    //set a boolean value
    let response = client
        .config("Pubsub.DisableSigning", Some("false"), Some(true), None)
        .await
        .expect("Config write failed");

    println!("Config: {}={}", response.key, response.value);

    //set a integer value
    let response = client
        .config("Datastore.StorageGCWatermark", Some("90"), None, Some(true))
        .await
        .expect("Config write failed");

    println!("Config: {}={}", response.key, response.value);
}
