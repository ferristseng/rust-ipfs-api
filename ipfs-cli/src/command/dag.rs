// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use clap::App;
use crate::command::CliCommand;
use futures::Future;

pub struct Command;

impl CliCommand for Command {
    const NAME: &'static str = "dag";

    fn signature<'a, 'b>() -> App<'a, 'b> {
        clap_app!(
            @subcommand dag =>
                (@setting SubcommandRequiredElseHelp)
                (@subcommand get =>
                    (about: "Get a dag node from IPFS")
                    (@arg KEY: +required "The key of the object to get")
                )
        )
    }

    handle!(
        client;
        ("get", args) => {
            let key = args.value_of("KEY").unwrap();

            client.dag_get(key).map(|dag| {
                println!();
                if let Some(data) = dag.data {
                    println!("  data                   :");
                    println!("{}", data);
                }
                println!("  links                  :");
                for link in dag.links {
                    println!("    {} ({}) ({:?})", link.name, link.size, link.cid);
                }
                println!();
            })
        }
    );
}
