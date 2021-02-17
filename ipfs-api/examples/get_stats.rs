// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use ipfs_api::IpfsClient;

// Creates an Ipfs client, and gets some stats about the Ipfs server.
//
#[cfg_attr(feature = "with-actix", actix_rt::main)]
#[cfg_attr(feature = "with-hyper", tokio::main)]
#[cfg_attr(feature = "with-reqwest", tokio::main)]
async fn main() {
    tracing_subscriber::fmt::init();

    eprintln!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    match client.stats_bitswap().await {
        Ok(bitswap_stats) => {
            eprintln!("bitswap stats:");
            eprintln!("  blocks recv: {}", bitswap_stats.blocks_received);
            eprintln!("  data   recv: {}", bitswap_stats.data_received);
            eprintln!("  blocks sent: {}", bitswap_stats.blocks_sent);
            eprintln!("  data   sent: {}", bitswap_stats.data_sent);
            eprintln!(
                "  peers:       {}",
                bitswap_stats.peers.join("\n               ")
            );
            eprintln!(
                "  wantlist:    {}",
                bitswap_stats.wantlist.join("\n               ")
            );
            eprintln!();
        }
        Err(e) => eprintln!("error getting bitswap stats: {}", e),
    }

    match client.stats_bw().await {
        Ok(bw_stats) => {
            eprintln!("bandwidth stats:");
            eprintln!("  total    in: {}", bw_stats.total_in);
            eprintln!("  total   out: {}", bw_stats.total_out);
            eprintln!("  rate     in: {}", bw_stats.rate_in);
            eprintln!("  rate    out: {}", bw_stats.rate_out);
            eprintln!();
        }
        Err(e) => eprintln!("error getting bandwidth stats: {}", e),
    }

    match client.stats_repo().await {
        Ok(repo_stats) => {
            eprintln!("repo stats:");
            eprintln!("  num    objs: {}", repo_stats.num_objects);
            eprintln!("  repo   size: {}", repo_stats.repo_size);
            eprintln!("  repo   path: {}", repo_stats.repo_path);
            eprintln!("  version    : {}", repo_stats.version);
        }
        Err(e) => eprintln!("error getting repo stats: {}", e),
    }
}
