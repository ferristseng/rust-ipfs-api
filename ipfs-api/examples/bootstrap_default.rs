extern crate ipfs_api;
extern crate tokio_core;

use ipfs_api::IpfsClient;
use tokio_core::reactor::Core;


// Lists clients in bootstrap list, then adds the default list, then removes
// them, and readds them.
//
fn main() {
    if let Ok(mut core) = Core::new() {
        println!("connecting to localhost:5001...");

        let client =
            IpfsClient::new(&core.handle(), "localhost", 5001).expect("expected a valid url");
        let bootstrap = client.bootstrap_list().expect("expected a valid request");
        let bootstrap = core.run(bootstrap).expect("expected a valid response");

        println!("current bootstrap peers:");
        for peer in bootstrap.peers {
            println!("  {}", peer);
        }

        println!("");
        println!("dropping all bootstrap peers...");

        let drop = client.bootstrap_rm_all().expect("expected a valid request");
        let drop = core.run(drop).expect("expected a valid response");

        println!("dropped:");
        for peer in drop.peers {
            println!("  {}", peer);
        }

        println!("");
        println!("adding default peers...");

        let add = client.bootstrap_add_default().expect("expected a valid request");
        let add = core.run(add).expect("expected a valid response");

        println!("added:");
        for peer in add.peers {
            println!("  {}", peer);
        }
    } else {
        println!("failed to create event loop");
    }
}
