// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use clap::App;
use ipfs_api::IpfsClient;
use tokio_core::reactor::Core;


pub fn signature<'a, 'b>() -> App<'a, 'b> {
    clap_app!(
        @subcommand version =>
            (about: "Show ipfs version information")
    )
}


pub fn handle(core: &mut Core, client: &IpfsClient) {
    let version = core.run(client.version()).expect(
        "expected response from API",
    );

    println!();
    println!("  version : {}", version.version);
    println!("  commit  : {}", version.commit);
    println!("  repo    : {}", version.repo);
    println!("  system  : {}", version.system);
    println!("  golang  : {}", version.golang);
    println!();
}
