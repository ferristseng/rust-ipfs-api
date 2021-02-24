// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use ipfs_api_examples::ipfs_api::{IpfsApi, IpfsClient};

// Creates an Ipfs client, read & set config values.
//
#[ipfs_api_examples::main]
async fn main() {
    tracing_subscriber::fmt::init();

    eprintln!("note: this must be run in the root of the project repository");
    eprintln!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    //read a string
    let response = client
        .config_get_string("Identity.PeerID")
        .await
        .expect("Config read failed");

    println!("Config: {}={}", response.key, response.value);

    //read a bool
    let response = client
        .config_get_bool("Datastore.HashOnRead")
        .await
        .expect("Config read failed");

    println!("Config: {}={}", response.key, response.value);

    //read a stringified json
    let response = client
        .config_get_json("Mounts")
        .await
        .expect("Config read failed");

    println!("Config: {}={}", response.key, response.value);

    //set a string value
    let response = client
        .config_set_string("Routing.Type", "dht")
        .await
        .expect("Config write failed");

    println!("Config: {}={}", response.key, response.value);

    //set a boolean value
    let response = client
        .config_set_bool("Pubsub.DisableSigning", false)
        .await
        .expect("Config write failed");

    println!("Config: {}={}", response.key, response.value);

    //set a json value
    let response = client
        .config_set_json("Discovery", r#"{"MDNS":{"Enabled":true,"Interval":10}}"#)
        .await
        .expect("Config write failed");

    println!("Config: {}={}", response.key, response.value);
}
