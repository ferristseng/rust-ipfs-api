// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

#[macro_use]
extern crate clap;

use crate::command::CliCommand;
use ipfs_api::IpfsClient;

mod command;

macro_rules! main {
    ($($cmd:ident);*) => {
        fn main() {
            let matches = clap_app!(
                app =>
                    (name: "IPFS CLI")
                    (about: "CLI for Go IPFS")
                    (version: crate_version!())
                    (author: "Ferris T. <ferristseng@fastmail.fm>")
                    (@setting SubcommandRequiredElseHelp)
                    $((subcommand: <command::$cmd::Command>::signature()))*
            ).get_matches();

            let client = IpfsClient::default();
            let command = match matches.subcommand() {
                $(
                (<command::$cmd::Command>::NAME, Some(args)) => {
                    <command::$cmd::Command>::handle(&client, args)
                }
                )*
                _ => unreachable!(),
            };

            hyper::rt::run(command);
        }
    }
}

main!(
    add;
    bitswap; block; bootstrap;
    cat; commands; config;
    dag; dht; diag; dns;
    file; files; filestore;
    shutdown;
    version
);
