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
    const NAME: &'static str = "bitswap";

    fn signature<'a, 'b>() -> App<'a, 'b> {
        clap_app!(
            @subcommand bitswap =>
                (@setting SubcommandRequiredElseHelp)
                (@subcommand ledger =>
                    (about: "Show the current ledger for a peer")
                    (@arg PEER: +required "Peer to inspect")
                )
                (@subcommand reprovide =>
                    (about: "Triggers a reprovide")
                )
                (@subcommand stat =>
                    (about: "Show some diagnostic information on the bitswap agent")
                )
                (@subcommand unwant =>
                    (about: "Remove a given block from your wantlist")
                    (@arg KEY: +required "Key of the block to remove")
                )
                (@subcommand wantlist =>
                    (about: "Shows blocks currently on the wantlist")
                )
        )
    }

    handle!(
        client;
        ("ledger", args) => {
            let peer = args.value_of("PEER").unwrap();

            client
                .bitswap_ledger(peer)
                .map(|ledger| {
                    println!();
                    println!("  peer      : {}", ledger.peer);
                    println!("  value     : {}", ledger.value);
                    println!("  sent      : {}", ledger.sent);
                    println!("  recv      : {}", ledger.recv);
                    println!("  exchanged : {}", ledger.exchanged);
                    println!();
                })
        },
        ("reprovide", _args) => {
            client.bitswap_reprovide().map(|_| ())
        },
        ("stat", _args) => {
            client
                .bitswap_stat()
                .map(|stat| {
                    println!();
                    println!("  provide_buf_len        : {}", stat.provide_buf_len);
                    println!("  wantlist               :");
                    for want in stat.wantlist {
                        println!("    {}", want);
                    }
                    println!("  peers                  :");
                    for peer in stat.peers {
                        println!("    {}", peer);
                    }
                    println!("  blocks_received        : {}", stat.blocks_received);
                    println!("  data_received          : {}", stat.data_received);
                    println!("  blocks_sent            : {}", stat.blocks_sent);
                    println!("  data_sent              : {}", stat.data_sent);
                    println!("  dup_blks_received      : {}", stat.dup_blks_received);
                    println!("  dup_data_received      : {}", stat.dup_data_received);
                    println!();
                })
        },
        ("unwant", args) => {
            let key = args.value_of("KEY").unwrap();

            client
                .bitswap_unwant(key)
                .map(|_| {
                    println!();
                    println!("  OK");
                    println!();
                })
        },
        ("wantlist", args) => {
            let peer = args.value_of("PEER");

            client
                .bitswap_wantlist(peer)
                .map(|wantlist| {
                    println!();
                    println!("  wantlist               :");
                    for key in wantlist.keys {
                        println!("    {}", key);
                    }
                    println!();
                })
        }
    );
}
