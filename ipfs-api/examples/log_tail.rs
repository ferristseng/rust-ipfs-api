// Copyright 2019 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use futures::{future, TryStreamExt};
use ipfs_api::IpfsClient;

// Tails the log of IPFS.
//
#[cfg_attr(feature = "with-actix", actix_rt::main)]
#[cfg_attr(any(feature = "with-hyper", feature = "with-reqwest"), tokio::main)]
async fn main() {
    tracing_subscriber::fmt::init();

    eprintln!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    if let Err(e) = client
        .log_tail()
        .try_for_each(|line| {
            println!("{}", line);

            future::ok(())
        })
        .await
    {
        eprintln!("error getting tail of log: {}", e);
    }
}
