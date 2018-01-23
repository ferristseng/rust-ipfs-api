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

pub fn handle(core: &mut Core, client: &IpfsClient, args: &ArgMatches) {
    match args.subcommand() {
        ("edit", _) => core.run(client.config_edit()).expect(EXPECTED_API),
        ("replace", Some(args)) => {
            let path = args.value_of("INPUT").unwrap();
            let config = File::open(path).expect(EXPECTED_FILE);

            core.run(client.config_replace(config)).expect(EXPECTED_API);
        }
        ("show", _) => {
            let config = core.run(client.config_show()).expect(EXPECTED_API);

            println!("{}", config);
        }
        _ => unreachable!(),
    }
}
