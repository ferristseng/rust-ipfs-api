// Copyright 2022 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::time::Duration;

use ipfs_api::response::VersionResponse;
use ipfs_api::{IpfsApi, IpfsClient, TryFromUri};

#[cfg(feature = "with-hyper")]
use hyper::client::HttpConnector;

#[cfg(feature = "with-hyper-tls")]
use hyper_tls::HttpsConnector;

cfg_if::cfg_if! {
    if #[cfg(feature = "with-actix")] {
        pub fn build_client(api_url: &str) -> IpfsClient {
            IpfsClient::from_str(api_url).unwrap()
        }
    } else if #[cfg(feature = "with-hyper-tls")] {
        pub fn build_client(api_url: &str) -> IpfsClient<HttpsConnector<HttpConnector>> {
            IpfsClient::from_str(api_url).unwrap()
        }
    } else if #[cfg(feature = "with-hyper")] {
        pub fn build_client(api_url: &str) -> IpfsClient<HttpConnector> {
            IpfsClient::from_str(api_url).unwrap()
        }
    }
}

pub async fn wait_for_server<C: IpfsApi>(client: &C) -> Result<VersionResponse, String> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "with-actix")] {
            const MESSAGE: &str = "Failed to connect to host: Connection refused";
        } else if #[cfg(feature = "with-hyper")] {
            const MESSAGE: &str = "tcp connect error";
        }
    }

    let mut attempts = 0;

    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;

        match client.version().await.map_err(|e| format!("{}", e)) {
            Ok(v) => {
                return Ok(v);
            }
            Err(msg) => {
                println!("Not ready yet: {}", msg);

                if msg.contains(MESSAGE) {
                    attempts += 1;

                    if attempts >= 4 {
                        return Err(format!("Already tried {} times", attempts));
                    }
                } else {
                    return Err(format!("Other failure: {}", msg));
                }
            }
        }
    }
}
