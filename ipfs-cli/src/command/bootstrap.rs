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

fn print_peers(peers: &[String]) {
    println!();
    println!("  peers                  :");
    for peer in peers {
        println!("    {}", peer);
    }
    println!();
}

pub struct Command;

impl CliCommand for Command {
    const NAME: &'static str = "bootstrap";

    fn signature<'a, 'b>() -> App<'a, 'b> {
        clap_app!(
            @subcommand bootstrap =>
                (@setting SubcommandRequiredElseHelp)
                (@subcommand add =>
                    (@setting SubcommandRequiredElseHelp)
                    (@subcommand default =>
                        (about: "Add default peers to the bootstrap list")
                    )
                )
                (@subcommand list =>
                    (about: "Show peers in the bootstrap list")
                )
                (@subcommand rm =>
                    (@setting SubcommandRequiredElseHelp)
                    (@subcommand all =>
                        (about: "Remove all peers from the bootstrap list")
                    )
                )
        )
    }

    handle!(
        client;
        ("add") => {
            ("default", _args) => {
                client.bootstrap_add_default().map(|peers| print_peers(&peers.peers))
            }
        },
        ("list", _args) => {
            client.bootstrap_list().map(|peers| print_peers(&peers.peers))
        },
        ("rm") => {
            ("all", _args) => {
                client.bootstrap_rm_all().map(|peers| print_peers(&peers.peers))
            }
        }
    );
}
