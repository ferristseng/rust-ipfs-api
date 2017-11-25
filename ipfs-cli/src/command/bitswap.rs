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
        @subcommand bitswap =>
            (@setting SubcommandRequiredElseHelp)
            (@subcommand ledger =>
                (about: "Show the current ledger for a peer")
                (@arg PEER: +required "Peer to inspect")
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


pub fn handle(core: &mut Core, client: &IpfsClient, bitswap: &ArgMatches) {
    match bitswap.subcommand() {
        ("ledger", Some(ref args)) => {
            let peer = args.value_of("PEER").unwrap();
            let ledger = core.run(client.bitswap_ledger(&peer)).expect(EXPECTED_API);

            println!("");
            println!("  peer      : {}", ledger.peer);
            println!("  value     : {}", ledger.value);
            println!("  sent      : {}", ledger.sent);
            println!("  recv      : {}", ledger.recv);
            println!("  exchanged : {}", ledger.exchanged);
            println!("");
        }
        ("stat", _) => {
            let stat = core.run(client.bitswap_stat()).expect(EXPECTED_API);

            println!("");
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
            println!("");
        }
        ("unwant", Some(ref args)) => {
            let key = args.value_of("KEY").unwrap();

            core.run(client.bitswap_unwant(&key)).expect(EXPECTED_API);

            println!("OK");
        }
        ("wantlist", Some(ref args)) => {
            let peer = args.value_of("PEER");
            let wantlist = core.run(client.bitswap_wantlist(peer)).expect(EXPECTED_API);

            println!("");
            println!("  wantlist               :");
            for key in wantlist.keys {
                println!("    {}", key);
            }
            println!("");
        }
        _ => unreachable!(),
    }
}
