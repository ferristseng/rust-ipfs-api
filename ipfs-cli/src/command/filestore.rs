// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use clap::{App, ArgMatches};
use command::EXPECTED_API;
use futures::stream::Stream;
use ipfs_api::{response, IpfsClient};
use tokio_core::reactor::Core;

pub fn signature<'a, 'b>() -> App<'a, 'b> {
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

fn print_filestore_object<E>(obj: &response::FilestoreObject) -> Result<(), E> {
    println!("  status                 : {}", obj.status);
    println!("  error_msg              : {}", obj.error_msg);
    println!("  key                    : {}", obj.key);
    println!("  file_path              : {}", obj.file_path);
    println!("  offset                 : {}", obj.offset);
    println!("  size                   : {}", obj.size);
    println!();

    Ok(())
}

pub fn handle(core: &mut Core, client: &IpfsClient, args: &ArgMatches) {
    match args.subcommand() {
        ("dups", _) => {
            let req = client.filestore_dups().for_each(|dup| {
                println!("  ref     : {}", dup.reference);
                println!("  err     : {}", dup.err);
                println!();

                Ok(())
            });

            println!();
            core.run(req).expect(EXPECTED_API);
            println!();
        }
        ("ls", Some(args)) => {
            let cid = args.value_of("CID");
            let req = client
                .filestore_ls(cid)
                .for_each(|res| print_filestore_object(&res));

            println!();
            core.run(req).expect(EXPECTED_API);
            println!();
        }
        ("verify", Some(args)) => {
            let cid = args.value_of("CID");
            let req = client
                .filestore_verify(cid)
                .for_each(|obj| print_filestore_object(&obj));

            println!();
            core.run(req).expect(EXPECTED_API);
            println!();
        }
        _ => unreachable!(),
    }
}
