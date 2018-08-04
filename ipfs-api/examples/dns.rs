// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

extern crate futures;
extern crate hyper;
extern crate ipfs_api;

use futures::Future;
use ipfs_api::IpfsClient;

// Creates an Ipfs client, resolves ipfs.io, and lists the contents of it.
//
fn main() {
    println!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    let req = client
        .dns("ipfs.io", false)
        .and_then(move |dns| {
            println!("dns resolves to ({})", &dns.path);
            println!();

            client.file_ls(&dns.path[..])
        }).map(|contents| {
            println!("found contents:");
            for directory in contents.objects.values() {
                for file in directory.links.iter() {
                    println!("[{}] ({} bytes)", file.name, file.size);
                }
            }
        }).map_err(|e| eprintln!("{}", e));

    hyper::rt::run(req);
}
