extern crate ipfs_api;
extern crate tokio_core;

use ipfs_api::IpfsClient;
use tokio_core::reactor::Core;


// Creates an Ipfs client, and gets information about your local address, and
// connected peers.
//
fn main() {
    if let Ok(mut core) = Core::new() {
        println!("connecting to localhost:5001...");

        let client =
            IpfsClient::new(&core.handle(), "localhost", 5001).expect("expected a valid url");

        let local = client.swarm_addrs_local().expect(
            "expected a valid request",
        );
        let local = core.run(local).expect("expected a valid response");

        println!("your addrs:");
        for addr in local.strings {
            println!("  {}", addr);
        }
        println!("");

        let connected = client.swarm_peers().expect("expected a valid request");
        let connected = core.run(connected).expect("expected a valid response");

        println!("connected:");
        for peer in connected.peers {
            let streams: Vec<&str> = peer.streams.iter().map(|s| &s.protocol[..]).collect();
            println!("  addr:     {}", peer.addr);
            println!("  peer:     {}", peer.peer);
            println!("  latency:  {}", peer.latency);
            println!("  muxer:    {}", peer.muxer);
            println!("  streams:  {}", streams.join(", "));
            println!("");
        }
    } else {
        println!("failed to create event loop");
    }
}
