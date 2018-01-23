// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use clap::{App, ArgMatches};
use command::EXPECTED_API;
use ipfs_api::IpfsClient;
use tokio_core::reactor::Core;

pub fn signature<'a, 'b>() -> App<'a, 'b> {
    clap_app!(
        @subcommand file =>
            (@setting SubcommandRequiredElseHelp)
            (@subcommand ls =>
                (about: "List directory contents for Unix filesystem objects")
                (@arg PATH: +required "THe path to list links from")
            )
    )
}

pub fn handle(core: &mut Core, client: &IpfsClient, args: &ArgMatches) {
    match args.subcommand() {
        ("ls", Some(args)) => {
            let path = args.value_of("PATH").unwrap();
            let ls = core.run(client.file_ls(path)).expect(EXPECTED_API);

            println!();
            println!("  arguments              :");
            for (k, arg) in ls.arguments {
                println!("    arg        : {}", k);
                println!("    value      : {}", arg);
                println!();
            }
            println!("  objects                :");
            for (k, obj) in ls.objects {
                println!("    key        : {}", k);
                println!("    hash       : {}", obj.hash);
                println!("    size       : {}", obj.size);
                println!("    type       : {}", obj.typ);
                println!("    links      :");
                for link in obj.links {
                    println!("      name       : {}", link.name);
                    println!("      hash       : {}", link.hash);
                    println!("      size       : {}", link.size);
                    if let Some(ref typ) = link.typ {
                        println!("      type       : {}", typ);
                    }
                    println!();
                }
            }
            println!();
        }
        _ => unreachable!(),
    }
}
