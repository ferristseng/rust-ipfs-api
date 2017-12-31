// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

extern crate ipfs_api;
extern crate tokio_core;

use ipfs_api::IpfsClient;
use tokio_core::reactor::Core;

// Creates an Ipfs client, resolves ipfs.io, and lists the contents of it.
//
fn main() {
    println!("connecting to localhost:5001...");

    let mut core = Core::new().expect("expected event loop");
    let client = IpfsClient::default(&core.handle());

    let req = client.dns("ipfs.io", false);
    let dns = core.run(req).expect("dns should resolve");

    println!("dns resolves to ({})", &dns.path);
    println!();

    let req = client.file_ls(&dns.path[..]);
    let contents = core.run(req).expect("api should return path contents");

    println!("found contents:");
    for directory in contents.objects.values() {
        for file in directory.links.iter() {
            println!("[{}] ({} bytes)", file.name, file.size);
        }
    }
}
