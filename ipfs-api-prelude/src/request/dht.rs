// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::request::ApiRequest;
use serde::Serialize;

#[derive(Serialize)]
pub struct DhtFindPeer<'a> {
    #[serde(rename = "arg")]
    pub peer: &'a str,
}

impl<'a> ApiRequest for DhtFindPeer<'a> {
    const PATH: &'static str = "/dht/findpeer";
}

#[derive(Serialize)]
pub struct DhtFindProvs<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,
}

impl<'a> ApiRequest for DhtFindProvs<'a> {
    const PATH: &'static str = "/dht/findprovs";
}

#[derive(Serialize)]
pub struct DhtGet<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,
}

impl<'a> ApiRequest for DhtGet<'a> {
    const PATH: &'static str = "/dht/get";
}

#[derive(Serialize)]
pub struct DhtProvide<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,
}

impl<'a> ApiRequest for DhtProvide<'a> {
    const PATH: &'static str = "/dht/provide";
}

#[derive(Serialize)]
pub struct DhtPut<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,

    #[serde(rename = "arg")]
    pub value: &'a str,
}

impl<'a> ApiRequest for DhtPut<'a> {
    const PATH: &'static str = "/dht/put";
}

#[derive(Serialize)]
pub struct DhtQuery<'a> {
    #[serde(rename = "arg")]
    pub peer: &'a str,
}

impl<'a> ApiRequest for DhtQuery<'a> {
    const PATH: &'static str = "/dht/query";
}
