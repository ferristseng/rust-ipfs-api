// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use clap::{App, ArgMatches};
use command::{verify_file, EXPECTED_API, EXPECTED_FILE};
use futures::stream::Stream;
use ipfs_api::IpfsClient;
use std::fs::File;
use std::io::{self, Write};
use tokio_core::reactor::Core;


pub fn signature<'a, 'b>() -> App<'a, 'b> {
    clap_app!(
        @subcommand files =>
            (@setting SubcommandRequiredElseHelp)
            (@subcommand cp =>
                (about: "Copy files in MFS")
                (@arg SRC: +required "The source object to copy")
                (@arg DEST: +required "The destination to copy the object to")
            )
            (@subcommand flush =>
                (about: "Flush a path's data to disk")
                (@arg PATH: "The path to flush")
            )
            (@subcommand ls =>
                (about: "List directories in MFS")
                (@arg PATH: "The past to list")
            )
            (@subcommand mkdir =>
                (about: "Make directories in MFS")
                (@arg PATH: +required "The directory to create")
                (@arg parents: -p --parents "Create parents if the directory \
                    does not already exist")
            )
            (@subcommand mv =>
                (about: "Move files in MFS")
                (@arg SRC: +required "The source object to move")
                (@arg DEST: +required "The destination to move the object to")
            )
            (@subcommand read =>
                (about: "Read a file in MFS")
                (@arg PATH: +required "The path to read")
            )
            (@subcommand rm =>
                (about: "Remove a file in MFS")
                (@arg PATH: +required "The file to remove")
                (@arg recursive: -r --recursive "Recursively remove directories")
            )
            (@subcommand stat =>
                (about: "Display status for a file in MFS")
                (@arg PATH: +required "The file to get status for")
            )
            (@subcommand write =>
                (about: "Write a file to MFS")
                (@arg DEST: +required "The path to write to")
                (@arg INPUT: +required {verify_file} "The file to write")
                (@arg create: --create "Create the file if it does not exist")
                (@arg truncate: --truncate "Truncate the file before writing")
            )
    )
}


pub fn handle(core: &mut Core, client: &IpfsClient, args: &ArgMatches) {
    match args.subcommand() {
        ("cp", Some(args)) => {
            let src = args.value_of("SRC").unwrap();
            let dest = args.value_of("DEST").unwrap();

            core.run(client.files_cp(src, dest)).expect(EXPECTED_API);

            println!();
            println!("  OK");
            println!();
        }
        ("flush", Some(args)) => {
            let path = args.value_of("PATH");

            core.run(client.files_flush(&path)).expect(EXPECTED_API);

            println!();
            println!("  OK");
            println!();
        }
        ("ls", Some(args)) => {
            let path = args.value_of("PATH");
            let ls = core.run(client.files_ls(&path)).expect(EXPECTED_API);

            println!();
            println!("  entries                :");
            for entry in ls.entries {
                println!("    name       : {}", entry.name);
                println!("    type       : {}", entry.typ);
                println!("    size       : {}", entry.size);
                println!("    hash       : {}", entry.hash);
                println!();
            }
            println!();
        }
        ("mkdir", Some(args)) => {
            let path = args.value_of("PATH").unwrap();

            core.run(client.files_mkdir(path, args.is_present("parents")))
                .expect(EXPECTED_API);

            println!();
            println!("  OK");
            println!();
        }
        ("mv", Some(args)) => {
            let src = args.value_of("SRC").unwrap();
            let dest = args.value_of("DEST").unwrap();

            core.run(client.files_mv(src, dest)).expect(EXPECTED_API);

            println!();
            println!("  OK");
            println!();
        }
        ("read", Some(args)) => {
            let path = args.value_of("PATH").unwrap();
            let req = client.files_read(path).for_each(|chunk| {
                io::stdout().write_all(&chunk).map_err(From::from)
            });

            core.run(req).expect(EXPECTED_API);
        }
        ("rm", Some(args)) => {
            let path = args.value_of("PATH").unwrap();
            let req = client.files_rm(path, args.is_present("recursive"));

            core.run(req).expect(EXPECTED_API);

            println!();
            println!("  OK");
            println!();
        }
        ("stat", Some(args)) => {
            let path = args.value_of("PATH").unwrap();
            let stat = core.run(client.files_stat(path)).expect(EXPECTED_API);

            println!();
            println!("  hash                   : {}", stat.hash);
            println!("  size                   : {}", stat.size);
            println!("  cumulative_size        : {}", stat.cumulative_size);
            println!("  blocks                 : {}", stat.blocks);
            println!("  type                   : {}", stat.typ);
            println!();
        }
        ("write", Some(args)) => {
            let dest = args.value_of("DEST").unwrap();
            let path = args.value_of("INPUT").unwrap();
            let file = File::open(path).expect(EXPECTED_FILE);
            let req = client.files_write(
                dest,
                args.is_present("create"),
                args.is_present("truncate"),
                file,
            );

            core.run(req).expect(EXPECTED_API);

            println!();
            println!("  OK");
            println!();
        }
        _ => unreachable!(),
    }
}
