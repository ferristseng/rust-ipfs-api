// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use clap::{App, ArgMatches};
use command::{verify_file, EXPECTED_API, EXPECTED_FILE};
use ipfs_api::IpfsClient;
use std::fs::File;
use tokio_core::reactor::Core;

pub fn signature<'a, 'b>() -> App<'a, 'b> {
    clap_app!(
        @subcommand add =>
            (about: "Add file to IPFS")
            (@arg INPUT: +required {verify_file} "File to add")
    )
}

pub fn handle(core: &mut Core, client: &IpfsClient, args: &ArgMatches) {
    let path = args.value_of("INPUT").unwrap();
    let file = File::open(path).expect(EXPECTED_FILE);
    let response = core.run(client.add(file)).expect(EXPECTED_API);

    println!();
    println!("  name    : {}", response.name);
    println!("  hash    : {}", response.hash);
    println!("  size    : {}", response.size);
    println!();
}
