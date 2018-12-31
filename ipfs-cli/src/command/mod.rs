// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use std::path::Path;
use clap::{App, ArgMatches};
use futures::Future;
use ipfs_api::IpfsClient;
use std::error::Error;
use std::fs;

pub type CommandExecutable = Box<Future<Item = (), Error = ()> + 'static + Send>;

pub const EXPECTED_FILE: &str = "expected to read input file";

/// Verifies that a path points to a file that exists, and not a directory.
///
pub fn verify_file<P>(path: P) -> Result<(), String> where P: AsRef<Path> {
    match fs::metadata(path) {
        Ok(ref metadata) if metadata.is_file() => Ok(()),
        Ok(_) => Err("file must not be a directory".into()),
        Err(e) => Err(e.description().into()),
    }
}

pub trait CliCommand {
    /// Name of the command.
    ///
    const NAME: &'static str;

    /// Returns the signature of the application.
    ///
    fn signature<'a, 'b>() -> App<'a, 'b>;

    /// Creates a future representing the request to make.
    ///
    fn handle(client: &IpfsClient, args: &ArgMatches) -> CommandExecutable;
}

macro_rules! handle_case {
    // Base macro case. Converts an expression into a boxed future.
    //
    ($run: expr) => {
        {
            let future = $run;

            return Box::new(future.map_err(|e| eprintln!("{}", e)))
        }
    };
    // Base case for nested subcommand (e.g. /bootstrap/add).
    //
    (
        $subcommand: ident;
        ($key: pat) => { $(($inner_key: pat, $args: ident) => $run: expr),* }
    ) => {
        if let ($key, Some(args)) = $subcommand {
            let inner_subcommand = args.subcommand();

            $(
                handle_case!(inner_subcommand; ($inner_key, $args) => $run);
            )*
        }
    };
    // Base case for subcommand.
    //
    (
        $subcommand: ident;
        ($key: pat, $args: pat) => $run: expr
    ) => {
        if let ($key, Some($args)) = $subcommand {
            handle_case!($run)
        }
    };
    // Recursive case for nested subcommand.
    //
    (
        $subcommand: ident;
        ($key: pat) => { $(($inner_key: pat, $args: ident) => $run: expr),* },
        $($rest_args: tt => $rest_run: tt),*
    ) => {
        handle_case!($subcommand; ($key) => { $(($inner_key, $args) => $run),* });

        $(
            handle_case!($subcommand; $rest_args => $rest_run);
        )*
    };
    // Recursive case fo subcommand.
    //
    (
        $subcommand: ident;
        ($key: pat, $args: pat) => $run: expr,
        $($rest_args: tt => $rest_run: tt),*
    ) => {
        handle_case!($subcommand; ($key, $args) => $run);

        $(
            handle_case!($subcommand; $rest_args => $rest_run);
        )*
    }
}

macro_rules! handle {
    // Command with no subcommands.
    //
    (
        ($args: ident, $client: ident) => $run: expr
    ) => {
        fn handle(
            client: &::ipfs_api::IpfsClient,
            args: &::clap::ArgMatches,
        ) -> ::command::CommandExecutable {
            let $args = args;
            let $client = client;

            handle_case!($run)
        }
    };
    // Command with one or more subcommands.
    //
    (
        $client: ident;
        $($args: tt => $run: tt),*
    ) => {
        fn handle(
            client: &::ipfs_api::IpfsClient,
            args: &::clap::ArgMatches,
        ) -> ::command::CommandExecutable {
            let $client = client;
            let subcommand = args.subcommand();

            handle_case!(subcommand; $($args => $run),*);

            unreachable!()
        }
    }
}

pub mod add;
pub mod bitswap;
pub mod block;
pub mod bootstrap;
pub mod cat;
pub mod commands;
pub mod config;
pub mod dag;
pub mod dht;
pub mod diag;
pub mod dns;
pub mod file;
pub mod files;
pub mod filestore;
pub mod shutdown;
pub mod version;
