// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use clap::{App, ArgMatches};
use command::EXPECTED_API;
use futures::stream::Stream;
use ipfs_api::IpfsClient;
use ipfs_api::response::DhtMessage;
use tokio_core::reactor::Core;


pub fn signature<'a, 'b>() -> App<'a, 'b> {
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
    )
}


fn print_dht_response(res: DhtMessage) {
    println!("");
    println!("  id                     : {}", res.id);
    println!("  type                   : {:?}", res.typ);
    println!("  responses              :");
    for peer_res in res.responses {
        println!("    id        : {}", peer_res.id);
        println!("    addrs     :");
        for addr in peer_res.addrs {
            println!("      {}", addr);
        }
        println!("");
    }
    println!("  extra                  : {}", res.extra);
    println!("");
}


pub fn handle(core: &mut Core, client: &IpfsClient, args: &ArgMatches) {
    match args.subcommand() {
        ("findpeer", Some(args)) => {
            let peer = args.value_of("PEER").unwrap();
            let req = client.dht_findpeer(&peer).for_each(|peer| {
                print_dht_response(peer);

                Ok(())
            });

            core.run(req).expect(EXPECTED_API);
        }
        ("findprovs", Some(args)) => {
            let key = args.value_of("KEY").unwrap();
            let req = client.dht_findprovs(&key).for_each(|peer| {
                print_dht_response(peer);

                Ok(())
            });

            core.run(req).expect(EXPECTED_API);
        }
        ("get", Some(args)) => {
            let key = args.value_of("KEY").unwrap();
            let req = client.dht_get(&key).for_each(|peer| {
                print_dht_response(peer);

                Ok(())
            });

            core.run(req).expect(EXPECTED_API);
        }
        ("provide", Some(args)) => {
            let key = args.value_of("KEY").unwrap();
            let req = client.dht_provide(&key).for_each(|peer| {
                print_dht_response(peer);

                Ok(())
            });

            core.run(req).expect(EXPECTED_API);
        }
        _ => unreachable!(),
    }
}
