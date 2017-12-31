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
use std::io::{self, Write};
use tokio_core::reactor::Core;


pub fn signature<'a, 'b>() -> App<'a, 'b> {
    clap_app!(
        @subcommand cat =>
            (about: "Show IPFS object data")
            (@arg PATH: +required "The path of the IPFS object to get")
    )
}


pub fn handle(core: &mut Core, client: &IpfsClient, args: &ArgMatches) {
    let path = args.value_of("PATH").unwrap();
    let req = client.cat(path).for_each(|chunk| {
        io::stdout().write_all(&chunk).map_err(From::from)
    });

    core.run(req).expect(EXPECTED_API);
}
