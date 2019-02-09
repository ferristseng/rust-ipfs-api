// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

#[cfg(feature = "actix")]
extern crate actix_web;
extern crate futures;
#[cfg(feature = "hyper")]
extern crate hyper;
extern crate ipfs_api;

use futures::Future;
use ipfs_api::IpfsClient;
use std::io::Cursor;

// Creates an Ipfs client, and replaces the config file with the default one.
//
fn main() {
    println!("note: this must be run in the root of the project repository");
    println!("connecting to localhost:5001...");

    let client = IpfsClient::default();
    let default_config = include_str!("default_config.json");
    let req = client
        .config_replace(Cursor::new(default_config))
        .map(|_| println!("replaced file"))
        .map_err(|e| println!("{}", e));

    #[cfg(feature = "hyper")]
    hyper::rt::run(req);
    #[cfg(feature = "actix")]
    actix_web::actix::run(|| {
        req.then(|_| {
            actix_web::actix::System::current().stop();
            Ok(())
        })
    });
}
