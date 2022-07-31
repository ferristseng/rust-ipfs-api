// Copyright 2021 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use futures::StreamExt;
use ipfs_api_examples::ipfs_api::{
    request::Ls, response::LsResponse, ApiError, BackendWithGlobalOptions, Error as IpfsError,
    GlobalOptions, IpfsApi, IpfsClient,
};
use std::process::exit;

// Creates an Ipfs client, and recursively checks whether argv[1] is cached on the local node.
//
#[ipfs_api_examples::main]
async fn main() {
    tracing_subscriber::fmt::init();

    eprintln!("note: this must be run in the root of the project repository");
    eprintln!("connecting to localhost:5001...");

    let client = BackendWithGlobalOptions::new(
        IpfsClient::default(),
        GlobalOptions::builder()
            .offline(true) // This is the entire trick!
            .build(),
    );
    // See also: https://discuss.ipfs.io/t/how-to-check-if-an-ipfs-object-is-on-your-local-node/1250/2
    // Note that if you have the file in mfs (ipfs files ...), ipfs files stat --with-local is likely faster

    let start_cid = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "QmUNLLsPACCz1vLxQVkXqqLX5R1X345qqfHbsf67hvA3Nn".into());
    let mut cids = vec![start_cid];

    while let Some(cid) = cids.pop() {
        println!("Checking {}", cid);
        let mut ls = client.ls_with_options(
            Ls::builder()
                .path(&cid)
                .resolve_type(false)
                .size(false)
                .build(),
        );
        while let Some(ls_chunk) = ls.next().await {
            match ls_chunk {
                Ok(LsResponse { objects, .. }) => {
                    for object in objects {
                        for file in object.links {
                            cids.push(file.hash)
                        }
                    }
                }
                Err(IpfsError::Api(ApiError { message, .. })) => {
                    // A better implementation would check for the semantic equivalent of "mergkledag: not found"
                    // I don't match on error messages by principle
                    println!("{} may be not local: {}", cid, message);
                    exit(1);
                }
                Err(e) => {
                    println!("Error while walking: {:?}", e);
                    exit(-1);
                }
            }
        }
    }
    println!("All local");
}
