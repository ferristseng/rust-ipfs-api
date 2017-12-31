// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

extern crate ipfs_api;
extern crate tokio_core;

use ipfs_api::IpfsClient;
use tokio_core::reactor::Core;

// Creates an Ipfs client, and gets some stats about the Ipfs server.
//
fn main() {
    println!("connecting to localhost:5001...");

    let mut core = Core::new().expect("expected event loop");
    let client = IpfsClient::default(&core.handle());

    let bitswap_stats = client.stats_bitswap();
    let bitswap_stats = core.run(bitswap_stats).expect("expected a valid response");

    let bw_stats = client.stats_bw();
    let bw_stats = core.run(bw_stats).expect("expected a valid response");

    let repo_stats = client.stats_repo();
    let repo_stats = core.run(repo_stats).expect("expected a valid response");

    println!("bitswap stats:");
    println!("  blocks recv: {}", bitswap_stats.blocks_received);
    println!("  data   recv: {}", bitswap_stats.data_received);
    println!("  blocks sent: {}", bitswap_stats.blocks_sent);
    println!("  data   sent: {}", bitswap_stats.data_sent);
    println!(
        "  peers:       {}",
        bitswap_stats.peers.join("\n               ")
    );
    println!(
        "  wantlist:    {}",
        bitswap_stats.wantlist.join("\n               ")
    );
    println!();
    println!("bandwidth stats:");
    println!("  total    in: {}", bw_stats.total_in);
    println!("  total   out: {}", bw_stats.total_out);
    println!("  rate     in: {}", bw_stats.rate_in);
    println!("  rate    out: {}", bw_stats.rate_out);
    println!();
    println!("repo stats:");
    println!("  num    objs: {}", repo_stats.num_objects);
    println!("  repo   size: {}", repo_stats.repo_size);
    println!("  repo   path: {}", repo_stats.repo_path);
    println!("  version    : {}", repo_stats.version);
}
