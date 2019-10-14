// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::command::CliCommand;
use clap::App;
use futures::{Future, Stream};
use std::io::{self, Write};

pub struct Command;

impl CliCommand for Command {
    const NAME: &'static str = "cat";

    fn signature<'a, 'b>() -> App<'a, 'b> {
        clap_app!(
            @subcommand cat =>
                (about: "Show IPFS object data")
                (@arg PATH: +required "The path of the IPFS object to get")
        )
    }

    handle!(
        (args, client) => {
            let path = args.value_of("PATH").unwrap();

            client
                .cat(&path)
                .for_each(|chunk| io::stdout().write_all(&chunk).map_err(From::from))
        }
    );
}
