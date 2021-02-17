// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use ipfs_api::{response, IpfsClient};
use std::fs::File;

fn print_stat(stat: response::FilesStatResponse) {
    eprintln!("  type     : {}", stat.typ);
    eprintln!("  hash     : {}", stat.hash);
    eprintln!("  size     : {}", stat.size);
    eprintln!("  cum. size: {}", stat.cumulative_size);
    eprintln!("  blocks   : {}", stat.blocks);
    eprintln!();
}

// Creates an Ipfs client, and makes some calls to the Mfs Api.
//
#[cfg_attr(feature = "with-actix", actix_rt::main)]
#[cfg_attr(feature = "with-hyper", tokio::main)]
#[cfg_attr(feature = "with-reqwest", tokio::main)]
async fn main() {
    tracing_subscriber::fmt::init();

    eprintln!("note: this must be run in the root of the project repository");
    eprintln!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    eprintln!("making /test...");
    eprintln!();

    if let Err(e) = client.files_mkdir("/test", false).await {
        eprintln!("error making /test: {}", e);
        return;
    }

    eprintln!("making dirs /test/does/not/exist/yet...");
    eprintln!();

    if let Err(e) = client.files_mkdir("/test/does/not/exist/yet", true).await {
        eprintln!("error making /test/does/not/exist/yet: {}", e);
        return;
    }

    eprintln!("getting status of /test/does...");
    eprintln!();

    match client.files_stat("/test/does").await {
        Ok(stat) => print_stat(stat),
        Err(e) => {
            eprintln!("error getting status of /test/does: {}", e);
            return;
        }
    }

    eprintln!("writing source file to /test/mfs.rs");
    eprintln!();

    let src = File::open(file!()).expect("could not read source file");

    if let Err(e) = client.files_write("/test/mfs.rs", true, true, src).await {
        eprintln!("error writing source file /test/mfs.rs: {}", e);
        return;
    }

    eprintln!("getting status of /test/mfs.rs...");
    eprintln!();

    match client.files_stat("/test/mfs.rs").await {
        Ok(stat) => print_stat(stat),
        Err(e) => {
            eprintln!("error getting status of /test/mfs.rs: {}", e);
            return;
        }
    }

    eprintln!("removing /test...");
    eprintln!();

    if let Err(e) = client.files_rm("/test", true).await {
        eprintln!("error removing /test: {}", e);
    }

    eprintln!("done!");
}
