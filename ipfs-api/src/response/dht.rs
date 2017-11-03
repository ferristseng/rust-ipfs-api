// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use response::serde;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DhtPeerResponse {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub addrs: Vec<String>,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DhtMessage {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "Type")]
    pub typ: isize,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub responses: Vec<DhtPeerResponse>,

    pub extra: String,
}


pub type DhtFindPeerResponse = DhtMessage;

pub type DhtFindProvsResponse = DhtMessage;

pub type DhtGetResponse = DhtMessage;

pub type DhtProvideResponse = DhtMessage;

pub type DhtPutResponse = DhtMessage;

pub type DhtQueryResponse = DhtMessage;
