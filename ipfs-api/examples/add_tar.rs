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
extern crate tar;

use futures::{Future, Stream};
use ipfs_api::IpfsClient;
use std::io::Cursor;
use tar::Builder;

// Creates an Ipfs client, and adds this source file to Ipfs.
//
fn main() {
    println!("note: this must be run in the root of the project repository");
    println!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    let mut buf = Vec::new();

    // Create a in-memory tar file with this source file as its contents.
    //
    {
        let mut builder = Builder::new(&mut buf);

        builder
            .append_path(file!())
            .expect("failed to create tar file");
        builder.finish().expect("failed to create tar file");
    }

    let cursor = Cursor::new(buf);
    let req = client
        .tar_add(cursor)
        .and_then(move |add| {
            println!("added tar file: {:?}", add);
            println!();

            client.tar_cat(&add.hash[..]).concat2()
        })
        .map(|cat| {
            println!("{}", String::from_utf8_lossy(&cat[..]));
            println!();
        })
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
