// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use clap::App;
use command::{verify_file, CliCommand, EXPECTED_FILE};
use futures::Future;
use std::fs::File;

pub struct Command;

impl CliCommand for Command {
    const NAME: &'static str = "add";

    fn signature<'a, 'b>() -> App<'a, 'b> {
        clap_app!(
            @subcommand add =>
                (about: "Add file to IPFS")
                (@arg INPUT: +required {verify_file} "File to add")
        )
    }

    handle!(
        (args, client) => {
            let path = args.value_of("INPUT").unwrap();
            let file = File::open(path).expect(EXPECTED_FILE);

            client
                .add(file)
                .map(|response| {
                    println!();
                    println!("  name    : {}", response.name);
                    println!("  hash    : {}", response.hash);
                    println!("  size    : {}", response.size);
                    println!();
                })
        }
    );
}
