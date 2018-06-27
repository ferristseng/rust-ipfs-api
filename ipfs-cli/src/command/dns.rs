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
    const NAME: &'static str = "dns";

    fn signature<'a, 'b>() -> App<'a, 'b> {
        clap_app!(
            @subcommand dns =>
                (about: "Resolve a DNS link")
                (@arg PATH: +required "The domain name to resolve")
                (@arg recursive: -r --recursive "Resolve until the result is not a DNS link")
        )
    }

    handle!(
        (args, client) => {
            let path = args.value_of("PATH").unwrap();

            client
                .dns(path, args.is_present("recursive"))
                .map(|res| {
                    println!();
                    println!("  path    : {}", res.path);
                    println!();
                })
        }
    );
}
