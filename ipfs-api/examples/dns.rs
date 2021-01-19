// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use ipfs_api::IpfsClient;

// Creates an Ipfs client, resolves ipfs.io, and lists the contents of it.
//
#[cfg_attr(feature = "with-actix", actix_rt::main)]
#[cfg_attr(feature = "with-hyper", tokio::main)]
async fn main() {
    eprintln!("connecting to localhost:5001...");

    let client = IpfsClient::default();

    let dns = match client.dns("ipfs.io", true).await {
        Ok(dns) => {
            eprintln!("dns resolves to ({})", &dns.path);
            eprintln!();

            dns
        }
        Err(e) => {
            eprintln!("error resolving dns: {}", e);
            return;
        }
    };

    match client.object_get(&dns.path[..]).await {
        Ok(contents) => {
            eprintln!("found contents:");
            for link in contents.links.iter() {
                eprintln!("[{}] ({} bytes)", link.name, link.size);
            }
        }
        Err(e) => eprintln!("error listing path: {}", e),
    }
}
