// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use clap::App;
use command::CliCommand;
use futures::{Future, Stream};
use ipfs_api::response::FilestoreObject;

fn print_filestore_object<E>(obj: FilestoreObject) -> Result<(), E> {
    println!("  status                 : {}", obj.status);
    println!("  error_msg              : {}", obj.error_msg);
    println!("  key                    : {}", obj.key);
    println!("  file_path              : {}", obj.file_path);
    println!("  offset                 : {}", obj.offset);
    println!("  size                   : {}", obj.size);
    println!();

    Ok(())
}

pub struct Command;

impl CliCommand for Command {
    const NAME: &'static str = "filestore";

    fn signature<'a, 'b>() -> App<'a, 'b> {
        clap_app!(
            @subcommand filestore =>
                (@setting SubcommandRequiredElseHelp)
                (@subcommand dups =>
                    (about: "List blocks that are both in the filestore and standard block storage")
                )
                (@subcommand ls =>
                    (about: "List objects in the filestore")
                    (@arg CID: "Cid of the object to list")
                )
                (@subcommand verify =>
                    (about: "Verify objects in the filestore")
                    (@arg CID: "Cid of the object to verify")
                )
        )
    }

    handle!(
        client;
        ("dups", _args) => {
            println!();

            client
                .filestore_dups()
                .for_each(|dup| {
                    println!("  ref     : {}", dup.reference);
                    println!("  err     : {}", dup.err);
                    println!();

                    Ok(())
                })
        },
        ("ls", args) => {
            let cid = args.value_of("CID");

            println!();

            client
                .filestore_ls(cid)
                .for_each(print_filestore_object)
        },
        ("verify", args) => {
            let cid = args.value_of("CID");

            println!();

            client
                .filestore_verify(cid)
                .for_each(print_filestore_object)
        }
    );
}
