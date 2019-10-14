// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::command::CliCommand;
use clap::{App, Arg, SubCommand};
use futures::Future;

pub struct Command;

impl CliCommand for Command {
    const NAME: &'static str = "diag";

    fn signature<'a, 'b>() -> App<'a, 'b> {
        // The clap macro does not allow hyphenated command names,
        // so the `set-time` command has to be manually instantiated.
        //
        let set_time_command = SubCommand::with_name("set-time")
            .about("Set how long to keep inactive requests in the log")
            .arg(
                Arg::with_name("TIME")
                    .required(true)
                    .index(1)
                    .help("Time to keep inactive requests in the log"),
            );

        clap_app!(
            @subcommand diag =>
                (@setting SubcommandRequiredElseHelp)
                (@subcommand cmds =>
                    (@setting SubcommandRequiredElseHelp)
                    (@subcommand clear =>
                        (about: "Clear inactive requests from the log")
                    )
                    (subcommand: set_time_command)
                )
                (@subcommand sys =>
                    (about: "Print system diagnostic information")
                )
        )
    }

    handle!(
        client;
        ("cmds") => {
            ("clear", _args) => {
                client
                    .diag_cmds_clear()
                    .map(|_| {
                        println!("");
                        println!("  OK");
                        println!("");
                    })
            },
            ("set-time", args) => {
                let time = args.value_of("TIME").unwrap();

                client
                    .diag_cmds_set_time(&time)
                    .map(|_| {
                        println!("");
                        println!("  OK");
                        println!("");
                    })
            }
        },
        ("sys", _args) => {
            client
                .diag_sys()
                .map(|sys| {
                    println!();
                    println!("  {}", sys);
                    println!();
                })
        }
    );
}
