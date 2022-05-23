// Copyright 2021 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use ipfs_api_backend_actix::{IpfsApi, IpfsClient, TryFromUri};
use std::io::Cursor;

#[actix_rt::main]
async fn main() {
    let client = IpfsClient::from_str("http://127.0.0.1:5001").unwrap();
    let data = Cursor::new("Test IPFS Server!");

    match client.add(data).await {
        Ok(res) => println!("{}", res.hash),
        Err(e) => eprintln!("error adding file: {}", e)
    }
}
