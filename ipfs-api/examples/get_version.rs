extern crate ipfs_api;
extern crate tokio_core;

use ipfs_api::IpfsClient;
use tokio_core::reactor::Core;

// Creates an Ipfs client, and gets the version of the Ipfs server.
//
fn main() {
    println!("connecting to localhost:5001...");

    let mut core = Core::new().expect("expected event loop");
    let client = IpfsClient::default(&core.handle());
    let req = client.version();
    let version = core.run(req).expect("expected a valid response");

    println!("version: {:?}", version.version);
}
