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
use ipfs_api::response::DhtMessage;

fn print_dht_response<E>(res: DhtMessage) -> Result<(), E> {
    println!();
    println!("  id                     : {}", res.id);
    println!("  type                   : {:?}", res.typ);
    println!("  responses              :");
    for peer_res in res.responses {
        println!("    id        : {}", peer_res.id);
        println!("    addrs     :");
        for addr in peer_res.addrs {
            println!("      {}", addr);
        }
        println!();
    }
    println!("  extra                  : {}", res.extra);
    println!();

    Ok(())
}

pub struct Command;

impl CliCommand for Command {
    const NAME: &'static str = "dht";

    fn signature<'a, 'b>() -> App<'a, 'b> {
        clap_app!(
            @subcommand dht =>
                (@setting SubcommandRequiredElseHelp)
                (@subcommand findpeer =>
                    (about: "Query the DHT for all of the multiaddresses associated with a Peer ID")
                    (@arg PEER: +required "Peer to search for")
                )
                (@subcommand findprovs =>
                    (about: "Find peers in the DHT that can provide the given key")
                    (@arg KEY: +required "Key to search for")
                )
                (@subcommand get =>
                    (about: "Given a key, query the DHT for its best value")
                    (@arg KEY: +required "The key search for")
                )
                (@subcommand provide =>
                    (about: "Announce to the network that you are providing the given values")
                    (@arg KEY: +required "The key you are providing")
                )
                (@subcommand put =>
                    (about: "Write a key/value pair to the DHT")
                    (@arg KEY: +required "The key to store the value at")
                    (@arg VALUE: +required "The value to store")
                )
                (@subcommand query =>
                    (about: "Find the closest peer to a given peer by querying the DHT")
                    (@arg PEER: +required "The peer to run the query against")
                )
        )
    }

    handle!(
        client;
        ("findpeer", args) => {
            let peer = args.value_of("PEER").unwrap();

            client.dht_findpeer(peer).for_each(print_dht_response)
        },
        ("findprovs", args) => {
            let key = args.value_of("KEY").unwrap();

            client.dht_findprovs(key).for_each(print_dht_response)
        },
        ("get", args) => {
            let key = args.value_of("KEY").unwrap();

            client.dht_get(key).for_each(print_dht_response)
        },
        ("provide", args) => {
            let key = args.value_of("KEY").unwrap();

            client.dht_provide(&key).for_each(print_dht_response)
        },
        ("put", args) => {
            let key = args.value_of("KEY").unwrap();
            let val = args.value_of("VALUE").unwrap();

            client.dht_put(key, val).for_each(print_dht_response)
        },
        ("query", args) => {
            let peer = args.value_of("PEER").unwrap();

            client.dht_query(peer).for_each(print_dht_response)
        }
    );
}
