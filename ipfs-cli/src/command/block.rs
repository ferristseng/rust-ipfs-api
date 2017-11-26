// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use clap::{App, ArgMatches};
use command::{verify_file, EXPECTED_API, EXPECTED_FILE};
use futures::stream::Stream;
use ipfs_api::IpfsClient;
use std::fs::File;
use std::io::{self, Write};
use tokio_core::reactor::Core;


pub fn signature<'a, 'b>() -> App<'a, 'b> {
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


pub fn handle(core: &mut Core, client: &IpfsClient, args: &ArgMatches) {
    match args.subcommand() {
        ("get", Some(args)) => {
            let key = args.value_of("KEY").unwrap();
            let req = client.block_get(key).for_each(|chunk| {
                io::stdout().write_all(&chunk).map_err(From::from)
            });

            core.run(req).expect(EXPECTED_API);
        }
        ("put", Some(args)) => {
            let path = args.value_of("INPUT").unwrap();
            let file = File::open(path).expect(EXPECTED_FILE);
            let block = core.run(client.block_put(file)).expect(EXPECTED_API);

            println!("");
            println!("  key     : {}", block.key);
            println!("  size    : {}", block.size);
            println!("");
        }
        ("rm", Some(args)) => {
            let key = args.value_of("KEY").unwrap();
            let rm = core.run(client.block_rm(key)).expect(EXPECTED_API);

            println!("");
            println!("  hash    : {}", rm.hash);
            if let Some(error) = rm.error {
                println!("  error   : {}", error);
            }
            println!("");
        }
        ("stat", Some(args)) => {
            let key = args.value_of("KEY").unwrap();
            let stat = core.run(client.block_stat(key)).expect(EXPECTED_API);

            println!("");
            println!("  key     : {}", stat.key);
            println!("  size    : {}", stat.size);
            println!("");
        }
        _ => unreachable!(),
    }

}
