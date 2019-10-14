// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::command::CliCommand;
use clap::App;
use futures::Future;

pub struct Command;

impl CliCommand for Command {
    const NAME: &'static str = "version";

    fn signature<'a, 'b>() -> App<'a, 'b> {
        clap_app!(
            @subcommand version =>
                (about: "Show ipfs version information")
        )
    }

    handle!(
        (_args, client) => {
            client.version().map(|version| {
                println!();
                println!("  version : {}", version.version);
                println!("  commit  : {}", version.commit);
                println!("  repo    : {}", version.repo);
                println!("  system  : {}", version.system);
                println!("  golang  : {}", version.golang);
                println!();
            })
        }
    );
}
