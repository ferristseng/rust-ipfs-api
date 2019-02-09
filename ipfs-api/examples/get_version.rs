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

// Creates an Ipfs client, and gets the version of the Ipfs server.
//
fn main() {
    println!("connecting to localhost:5001...");

    let client = IpfsClient::default();
    let req = client
        .version()
        .map(|version| println!("version: {:?}", version.version));

    let fut = req.map_err(|e| eprintln!("{}", e));
    
    #[cfg(feature = "hyper")]
    hyper::rt::run(fut);
    #[cfg(feature = "actix")]
    actix_web::actix::run(|| {
        fut.then(|_| {
            actix_web::actix::System::current().stop();
            Ok(())
        })
    });
}
