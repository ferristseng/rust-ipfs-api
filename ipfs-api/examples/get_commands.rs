// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

#[cfg(feature = "actix")]
extern crate actix_web;
extern crate futures;
#[cfg(feature = "hyper")]
extern crate hyper;
extern crate ipfs_api;

use futures::Future;
use ipfs_api::{response, IpfsClient};

fn print_recursive(indent: usize, cmd: &response::CommandsResponse) {
    let cmd_indent = " ".repeat(indent * 4);
    let opt_indent = " ".repeat((indent + 1) * 4);

    println!("{}[{}]", cmd_indent, cmd.name);

    if cmd.options.len() > 0 {
        println!("{}* options:", cmd_indent);
        for options in cmd.options.iter() {
            println!("{}{}", opt_indent, &options.names[..].join(", "));
        }
    }

    if cmd.subcommands.len() > 0 {
        println!("{}- subcommands:", cmd_indent);
        for subcommand in cmd.subcommands.iter() {
            print_recursive(indent + 1, subcommand);
        }
    }
}

// Creates an Ipfs client, and gets a list of available commands from the
// Ipfs server.
//
fn main() {
    println!("connecting to localhost:5001...");

    let client = IpfsClient::default();
    let req = client
        .commands()
        .map(|commands| print_recursive(0, &commands))
        .map_err(|e| eprintln!("{}", e));

    #[cfg(feature = "hyper")]
    hyper::rt::run(req);
    #[cfg(feature = "actix")]
    actix_web::actix::run(|| {
        req.then(|_| {
            actix_web::actix::System::current().stop();
            Ok(())
        })
    });
}
