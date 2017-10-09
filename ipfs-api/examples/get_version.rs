extern crate ipfs_api;
extern crate tokio_core;

use ipfs_api::IpfsClient;
use tokio_core::reactor::Core;


// Creates an Ipfs client, and gets the version of the Ipfs server.
//
fn main() {
    if let Ok(mut core) = Core::new() {
        println!("connecting to localhost:5001...");

        let client =
            IpfsClient::new(&core.handle(), "localhost", 5001).expect("expected a valid url");
        let req = client.version().expect("expected a valid request");
        let version = core.run(req).expect("expected a valid response");

        println!("version: {:?}", version.version);
    } else {
        println!("failed to create event loop");
    }
}
