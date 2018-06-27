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
    const NAME: &'static str = "shutdown";

    fn signature<'a, 'b>() -> App<'a, 'b> {
        clap_app!(
            @subcommand shutdown =>
                (about: "Shutdown IPFS daemon")
        )
    }

    handle!(
        (_args, client) => {
            client.shutdown().map(|_| ())
        }
    );
}
