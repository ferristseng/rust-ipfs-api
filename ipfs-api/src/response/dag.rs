// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use response::serde;
use std::collections::HashMap;


#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct DagIpfsHeader {
    pub name: String,
    pub size: u64,

    #[serde(deserialize_with = "serde::deserialize_hashmap")]
    pub cid: HashMap<String, String>,
}


#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DagGetResponse {
    pub data: Option<String>,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub links: Vec<DagIpfsHeader>,
}


#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub struct DagPutResponse {
    pub cid: String,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_dag_get_0, DagGetResponse);
}
