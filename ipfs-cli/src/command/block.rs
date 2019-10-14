// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::command::{verify_file, CliCommand, EXPECTED_FILE};
use clap::App;
use futures::{Future, Stream};
use std::fs::File;
use std::io::{self, Write};

pub struct Command;

impl CliCommand for Command {
    const NAME: &'static str = "block";

    fn signature<'a, 'b>() -> App<'a, 'b> {
        clap_app!(
            @subcommand block =>
                (@setting SubcommandRequiredElseHelp)
                (@subcommand get =>
                    (about: "Get a raw IPFS block")
                    (@arg KEY: +required "The base58 multihash of an existing block")
                )
                (@subcommand put =>
                    (about: "Store a file as an IPFS block")
                    (@arg INPUT: +required {verify_file} "The file to store as an IPFS block")
                )
                (@subcommand rm =>
                    (about: "Removes an IPFS block")
                    (@arg KEY: +required "The base58 multihash of a block to remove")
                )
                (@subcommand stat =>
                    (about: "Get information about a raw IPFS block")
                    (@arg KEY: +required "The base58 multihash of the block to retrieve")
                )
        )
    }

    handle!(
        client;
        ("get", args) => {
            let key = args.value_of("KEY").unwrap();

            client
                .block_get(key)
                .for_each(|chunk| io::stdout().write_all(&chunk).map_err(From::from))
        },
        ("put", args) => {
            let path = args.value_of("INPUT").unwrap();
            let file = File::open(path).expect(EXPECTED_FILE);

            client
                .block_put(file)
                .map(|block| {
                    println!();
                    println!("  key     : {}", block.key);
                    println!("  size    : {}", block.size);
                    println!();
                })
        },
        ("rm", args) => {
            let key = args.value_of("KEY").unwrap();

            client
                .block_rm(key)
                .map(|rm| {
                    println!();
                    println!("  hash    : {}", rm.hash);
                    if let Some(error) = rm.error {
                        println!("  error   : {}", error);
                    }
                    println!();
                })
        },
        ("stat", args) => {
            let key = args.value_of("KEY").unwrap();

            client
                .block_stat(key)
                .map(|stat| {
                    println!();
                    println!("  key     : {}", stat.key);
                    println!("  size    : {}", stat.size);
                    println!();
                })
        }
    );
}
