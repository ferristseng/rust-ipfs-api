// Copyright 2022 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[path = "test_support/lib.rs"]
mod test_support;

use test_support::client::{build_client, wait_for_server};
use test_support::container::{IpfsContainer, NginxContainer};
use test_support::images;
use test_support::rt::run_async;

use ipfs_api_versions::test_current_image;

#[test_current_image]
fn test_basic_auth(image_name: &str, image_tag: &str) {
    run_async(run_test(image_name, image_tag));
}

async fn run_test(image_name: &str, image_tag: &str) {
    let expected_version = images::extract_version(image_tag);

    let container = IpfsContainer::new("test_basic_auth_ipfs", image_name, image_tag)
        .await
        .unwrap();

    {
        let api_url = format!("http://{}:5001", container.ip);
        let client = build_client(&api_url);
        let version = wait_for_server(&client).await.unwrap();

        assert_eq!(expected_version, version.version);
    }

    let nginx = NginxContainer::new("test_basic_auth_nginx", &container.ip)
        .await
        .unwrap();

    println!("Waiting for nginx");

    nginx.wait_for().await.unwrap();

    println!("Connecting to IPFS through nginx");

    {
        let api_url = format!("http://{}", nginx.ip);
        let client = build_client(&api_url).with_credentials(&nginx.username, &nginx.password);
        let version = wait_for_server(&client).await.unwrap();

        assert_eq!(expected_version, version.version);
    }

    println!("Tearing down nginx");

    nginx.teardown().await.unwrap();

    println!("Tearing down ipfs");

    container.teardown().await.unwrap();
}
