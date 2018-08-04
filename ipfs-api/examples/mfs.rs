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
use ipfs_api::{response, IpfsClient};
use std::fs::File;

fn print_stat(stat: response::FilesStatResponse) {
    println!("  type     : {}", stat.typ);
    println!("  hash     : {}", stat.hash);
    println!("  size     : {}", stat.size);
    println!("  cum. size: {}", stat.cumulative_size);
    println!("  blocks   : {}", stat.blocks);
    println!();
}

// Creates an Ipfs client, and makes some calls to the Mfs Api.
//
fn main() {
    println!("note: this must be run in the root of the project repository");
    println!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    println!("making /test...");
    println!();

    let mkdir = client.files_mkdir("/test", false);
    let mkdir_recursive = client.files_mkdir("/test/does/not/exist/yet", true);

    let file_stat = client.files_stat("/test/does");

    let src = File::open(file!()).expect("could not read source file");
    let file_write = client.files_write("/test/mfs.rs", true, true, src);

    let file_write_stat = client.files_stat("/test/mfs.rs");

    let file_rm = client.files_rm("/test", true);

    hyper::rt::run(
        mkdir
            .and_then(|_| {
                println!("making dirs /test/does/not/exist/yet...");
                println!();

                mkdir_recursive
            }).and_then(|_| {
                println!("getting status of /test/does...");
                println!();

                file_stat
            }).and_then(|stat| {
                print_stat(stat);

                println!("writing source file to /test/mfs.rs");
                println!();

                file_write
            }).and_then(|_| file_write_stat)
            .and_then(|stat| {
                print_stat(stat);

                println!("removing /test...");
                println!();

                file_rm
            }).map(|_| println!("done!"))
            .map_err(|e| eprintln!("{}", e)),
    )
}
