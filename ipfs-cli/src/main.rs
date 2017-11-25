// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

#[macro_use]
extern crate clap;
extern crate ipfs_api;
extern crate tokio_core;

use ipfs_api::IpfsClient;
use tokio_core::reactor::Core;

mod command;

fn main() {
    let matches = clap_app!(
        app =>
            (name: "IPFS CLI")
            (about: "CLI for Go IPFS")
            (version: crate_version!())
            (author: "Ferris T. <ferristseng@fastmail.fm>")
            (@setting SubcommandRequiredElseHelp)
            (subcommand: command::add::signature())
            (subcommand: command::bitswap::signature())
            (subcommand: command::block::signature())
            (subcommand: command::bootstrap::signature())
            (subcommand: command::version::signature())
    ).get_matches();

    let mut core = Core::new().expect("expected event loop");
    let client = IpfsClient::default(&core.handle());

    match matches.subcommand() {
        ("add", Some(args)) => command::add::handle(&mut core, &client, args),
        ("bitswap", Some(bitswap)) => command::bitswap::handle(&mut core, &client, &bitswap),
        ("block", Some(block)) => command::block::handle(&mut core, &client, &block),
        ("bootstrap", Some(bootstrap)) => {
            command::bootstrap::handle(&mut core, &client, &bootstrap)
        }
        ("version", _) => command::version::handle(&mut core, &client),
        _ => unreachable!(),
    }
}
