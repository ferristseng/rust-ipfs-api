#[macro_use]
extern crate clap;
extern crate ipfs_api;
extern crate tokio_core;

use ipfs_api::IpfsClient;
use std::fs::File;
use tokio_core::reactor::Core;

fn main() {
    let matches = clap_app!(
        app =>
            (name: "IPFS CLI")
            (about: "CLI for Go IPFS")
            (version: crate_version!())
            (author: "Ferris T. <ferristseng@fastmail.fm>")
            (@setting SubcommandRequiredElseHelp)
            (@subcommand add =>
                (about: "Add file to ipfs")
                (@arg INPUT: +required "File to add")
            )
            (@subcommand bitswap =>
                (@setting SubcommandRequiredElseHelp)
                (@subcommand ledger =>
                    (about: "Show the current ledger for a peer")
                    (@arg PEER: +required "Peer to inspect")
                )
            )
            (@subcommand version =>
                (about: "Show ipfs version information")
            )
    ).get_matches();

    let mut core = Core::new().expect("expected event loop");
    let client = IpfsClient::default(&core.handle());

    match matches.subcommand() {
        ("add", Some(args)) => {
            let path = args.value_of("INPUT").unwrap();
            let file = File::open(path).expect("expected to read input file");
            let metadata = file.metadata().expect("expected to read file's metadata");

            if !metadata.is_file() {
                panic!("input must be a file not directory");
            }

            let response = core.run(client.add(file)).expect(
                "expected response from API",
            );

            println!("");
            println!("  name    : {}", response.name);
            println!("  hash    : {}", response.hash);
            println!("  size    : {}", response.size);
            println!("");
        }
        ("bitswap", Some(bitswap)) => {
            match bitswap.subcommand() {
                ("ledger", Some(ledger)) => {
                    let peer = ledger.value_of("PEER").unwrap();
                    let ledger = core.run(client.bitswap_ledger(&peer)).expect(
                        "expected response from API",
                    );

                    println!("");
                    println!("  peer      : {}", ledger.peer);
                    println!("  value     : {}", ledger.value);
                    println!("  sent      : {}", ledger.sent);
                    println!("  recv      : {}", ledger.recv);
                    println!("  exchanged : {}", ledger.exchanged);
                    println!("");
                }
                _ => unreachable!(),
            }
        }
        ("version", _) => {
            let version = core.run(client.version()).expect(
                "expected response from API",
            );

            println!("");
            println!("  version : {}", version.version);
            println!("  commit  : {}", version.commit);
            println!("  repo    : {}", version.repo);
            println!("  system  : {}", version.system);
            println!("  golang  : {}", version.golang);
            println!("");
        }
        _ => unreachable!(),
    }
}
