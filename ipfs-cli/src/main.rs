// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

#[macro_use]
extern crate clap;
extern crate futures;
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
            (subcommand: command::cat::signature())
            (subcommand: command::commands::signature())
            (subcommand: command::config::signature())
            (subcommand: command::dag::signature())
            (subcommand: command::dht::signature())
            (subcommand: command::version::signature())
    ).get_matches();

    let mut core = Core::new().expect("expected event loop");
    let client = IpfsClient::default(&core.handle());

    match matches.subcommand() {
        ("add", Some(args)) => command::add::handle(&mut core, &client, &args),
        ("bitswap", Some(args)) => command::bitswap::handle(&mut core, &client, &args),
        ("block", Some(args)) => command::block::handle(&mut core, &client, &args),
        ("bootstrap", Some(args)) => command::bootstrap::handle(&mut core, &client, &args),
        ("cat", Some(args)) => command::cat::handle(&mut core, &client, &args),
        ("commands", _) => command::commands::handle(&mut core, &client),
        ("config", Some(args)) => command::config::handle(&mut core, &client, &args),
        ("dag", Some(args)) => command::dag::handle(&mut core, &client, &args),
        ("dht", Some(args)) => command::dht::handle(&mut core, &client, &args),
        ("version", _) => command::version::handle(&mut core, &client),
        _ => unreachable!(),
    }
}
