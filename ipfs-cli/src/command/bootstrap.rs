// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use clap::{App, ArgMatches};
use command::EXPECTED_API;
use ipfs_api::IpfsClient;
use tokio_core::reactor::Core;

pub fn signature<'a, 'b>() -> App<'a, 'b> {
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

fn print_peers(peers: &[String]) {
    println!();
    println!("  peers                  :");
    for peer in peers {
        println!("    {}", peer);
    }
    println!();
}

pub fn handle(core: &mut Core, client: &IpfsClient, args: &ArgMatches) {
    match args.subcommand() {
        ("add", Some(add)) => match add.subcommand() {
            ("default", _) => {
                let peers = core.run(client.bootstrap_add_default())
                    .expect(EXPECTED_API);

                print_peers(&peers.peers);
            }
            _ => unreachable!(),
        },
        ("list", _) => {
            let peers = core.run(client.bootstrap_list()).expect(EXPECTED_API);

            print_peers(&peers.peers);
        }
        ("rm", Some(rm)) => match rm.subcommand() {
            ("all", _) => {
                let peers = core.run(client.bootstrap_rm_all()).expect(EXPECTED_API);

                print_peers(&peers.peers);
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
