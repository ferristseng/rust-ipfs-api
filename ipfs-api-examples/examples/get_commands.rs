// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use ipfs_api_examples::ipfs_api::{response, IpfsApi, IpfsClient};

fn print_recursive(indent: usize, cmd: &response::CommandsResponse) {
    let cmd_indent = " ".repeat(indent * 4);
    let opt_indent = " ".repeat((indent + 1) * 4);

    eprintln!("{}[{}]", cmd_indent, cmd.name);

    if !cmd.options.is_empty() {
        eprintln!("{}* options:", cmd_indent);
        for options in cmd.options.iter() {
            eprintln!("{}{}", opt_indent, &options.names[..].join(", "));
        }
    }

    if !cmd.subcommands.is_empty() {
        eprintln!("{}- subcommands:", cmd_indent);
        for subcommand in cmd.subcommands.iter() {
            print_recursive(indent + 1, subcommand);
        }
    }
}

// Creates an Ipfs client, and gets a list of available commands from the
// Ipfs server.
//
#[ipfs_api_examples::main]
async fn main() {
    tracing_subscriber::fmt::init();

    eprintln!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    match client.commands().await {
        Ok(commands) => print_recursive(0, &commands),
        Err(e) => eprintln!("error getting commands: {}", e),
    }
}
