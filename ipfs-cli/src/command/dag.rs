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
        @subcommand dag =>
            (@setting SubcommandRequiredElseHelp)
            (@subcommand get =>
                (about: "Get a dag node from IPFS")
                (@arg KEY: +required "The key of the object to get")
            )
    )
}


pub fn handle(core: &mut Core, client: &IpfsClient, args: &ArgMatches) {
    match args.subcommand() {
        ("get", Some(args)) => {
            let key = args.value_of("KEY").unwrap();
            let dag = core.run(client.dag_get(key)).expect(EXPECTED_API);

            println!("");
            if let Some(data) = dag.data {
                println!("  data                   :");
                println!("{}", data);
            }
            println!("  links                  :");
            for link in dag.links {
                println!("    {} ({}) ({:?})", link.name, link.size, link.cid);
            }
            println!("");
        }
        _ => unreachable!(),
    }
}
