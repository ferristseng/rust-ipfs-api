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
        @subcommand dns =>
            (about: "Resolve a DNS link")
            (@arg PATH: +required "The domain name to resolve")
            (@arg recursive: -r --recursive "Resolve until the result is not a DNS link")
    )
}


pub fn handle(core: &mut Core, client: &IpfsClient, args: &ArgMatches) {
    let path = args.value_of("PATH").unwrap();
    let req = client.dns(path, args.is_present("recursive"));
    let res = core.run(req).expect(EXPECTED_API);

    println!();
    println!("  path    : {}", res.path);
    println!();
}
