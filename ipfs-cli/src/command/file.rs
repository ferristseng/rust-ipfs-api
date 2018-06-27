// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use clap::App;
use command::CliCommand;
use futures::Future;

pub struct Command;

impl CliCommand for Command {
    const NAME: &'static str = "file";

    fn signature<'a, 'b>() -> App<'a, 'b> {
        clap_app!(
            @subcommand file =>
                (@setting SubcommandRequiredElseHelp)
                (@subcommand ls =>
                    (about: "List directory contents for Unix filesystem objects")
                    (@arg PATH: +required "The path to list links from")
                )
        )
    }

    handle!(
        client;
        ("ls", args) => {
            let path = args.value_of("PATH").unwrap();

            client
                .file_ls(path)
                .map(|ls| {
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
                })
        }
    );
}
