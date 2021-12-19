// Copyright 2021 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use ipfs_api_examples::ipfs_api::{IpfsApi, IpfsClient};
use std::process::{Command, Stdio};

// Creates an Ipfs client, and pipes the result of a command into Ipfs.
//
#[ipfs_api_examples::main]
async fn main() {
    tracing_subscriber::fmt::init();

    eprintln!("note: this must be run in the root of the project repository");
    eprintln!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    let mut output = Command::new("cat")
        .arg("README.md")
        .stdout(Stdio::piped())
        .spawn()
        .expect("expected to run `cat`");

    match client.add(output.stdout.take().unwrap()).await {
        Ok(file) => eprintln!("added file: {:?}", file),
        Err(e) => eprintln!("error adding file: {}", e),
    }

    let result = output.wait().expect("expected `cat` to finish");

    eprintln!("`cat` exited with {:?}", result.code());
}
