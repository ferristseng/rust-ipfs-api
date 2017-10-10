extern crate ipfs_api;
extern crate tokio_core;

use ipfs_api::IpfsClient;
use std::fs::File;
use tokio_core::reactor::Core;


// Creates an Ipfs client, and adds this source file to Ipfs.
//
fn main() {
    if let Ok(mut core) = Core::new() {
        println!("note: this must be run in the root of the project repository");
        println!("connecting to localhost:5001...");

        let client =
            IpfsClient::new(&core.handle(), "localhost", 5001).expect("expected a valid url");
        let file = File::open(file!()).expect("could not read source file");
        let req = client.add(file).expect("expected a valid request");
        let add = core.run(req).expect("expected a valid response");

        println!("added file: {:?}", add);
    } else {
        println!("failed to create event loop");
    }
}
