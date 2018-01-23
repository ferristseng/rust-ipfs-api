// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use clap::App;
use command::EXPECTED_API;
use ipfs_api::IpfsClient;
use ipfs_api::response::CommandsResponse;
use tokio_core::reactor::Core;

pub fn signature<'a, 'b>() -> App<'a, 'b> {
    clap_app!(
        @subcommand commands =>
            (about: "List all available commands")
    )
}

fn recursive_print_commands(cmd: CommandsResponse, stack: &mut Vec<String>) {
    if cmd.subcommands.is_empty() {
        println!("  {} {}", stack.join(" "), cmd.name);
    } else {
        let (name, subcommands) = (cmd.name, cmd.subcommands);

        stack.push(name);

        for subcommand in subcommands {
            recursive_print_commands(subcommand, stack);
        }

        stack.pop();
    }
}

pub fn handle(core: &mut Core, client: &IpfsClient) {
    let commands = core.run(client.commands()).expect(EXPECTED_API);

    println!();
    recursive_print_commands(commands, &mut Vec::new());
    println!();
}
