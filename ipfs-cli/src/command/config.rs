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
    const NAME: &'static str = "config";

    fn signature<'a, 'b>() -> App<'a, 'b> {
        clap_app!(
            @subcommand config =>
                (@setting SubcommandRequiredElseHelp)
                (@subcommand edit =>
                    (about: "Open the config file for editing")
                )
                (@subcommand replace =>
                    (about: "Replace the config file")
                    (@arg INPUT: +required {verify_file} "Config file to replace with")
                )
                (@subcommand show =>
                    (about: "Show the config file")
                )
        )
    }

    handle!(
        client;
        ("edit", _args) => {
            client.config_edit().map(|_| ())
        },
        ("replace", args) => {
            let path = args.value_of("INPUT").unwrap();
            let config = File::open(path).expect(EXPECTED_FILE);

            client.config_replace(config).map(|_| ())
        },
        ("show", _args) => {
            client.config_show().map(|config| println!("{}", config))
        }
    );
}
