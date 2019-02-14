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
use std::fs::File;

// Creates an Ipfs client, and adds this source file to Ipfs.
//
fn main() {
    println!("note: this must be run in the root of the project repository");
    println!("connecting to localhost:5001...");

    let client = IpfsClient::default();
    let file = File::open(file!()).expect("could not read source file");
    let req = client
        .add(file)
        .map(|add| println!("added file: {:?}", add))
        .map_err(|e| eprintln!("{}", e));

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
