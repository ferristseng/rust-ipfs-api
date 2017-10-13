extern crate ipfs_api;
extern crate tokio_core;

use ipfs_api::IpfsClient;
use tokio_core::reactor::Core;

// Creates an Ipfs client, and gets information about your local address, and
// connected peers.
//
fn main() {
    println!("connecting to localhost:5001...");

    let mut core = Core::new().expect("expected event loop");
    let client = IpfsClient::default(&core.handle());

    let local = client.swarm_addrs_local();
    let local = core.run(local).expect("expected a valid response");

    println!("");
    println!("your addrs:");
    for addr in local.strings {
        println!("  {}", addr);
    }

    let connected = client.swarm_peers();
    let connected = core.run(connected).expect("expected a valid response");

    println!("");
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
}
