// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::response::{serde, IpfsHeader};
use crate::serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectDiff {
    #[serde(rename = "Type")]
    pub typ: u64,

    pub path: String,

    #[serde(deserialize_with = "serde::deserialize_hashmap")]
    pub before: HashMap<String, String>,

    #[serde(deserialize_with = "serde::deserialize_hashmap")]
    pub after: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectDiffResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub changes: Vec<ObjectDiff>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectGetResponse {
    pub data: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub links: Vec<IpfsHeader>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectLinksResponse {
    pub hash: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub links: Vec<IpfsHeader>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectNewResponse {
    pub hash: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    #[serde(default)]
    pub links: Vec<IpfsHeader>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectPatchAddLinkResponse {
    pub hash: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    #[serde(default)]
    pub links: Vec<IpfsHeader>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectPatchAppendDataResponse {
    pub hash: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub links: Vec<IpfsHeader>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectPatchRmLinkResponse {
    pub hash: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub links: Vec<IpfsHeader>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectPatchSetDataResponse {
    pub hash: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub links: Vec<IpfsHeader>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectPutResponse {
    pub hash: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub links: Vec<IpfsHeader>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectStatResponse {
    pub hash: String,
    pub num_links: u64,
    pub block_size: u64,
    pub links_size: u64,
    pub data_size: u64,
    pub cumulative_size: u64,
}

#[cfg(test)]
mod tests {
    deserialize_test!(v0_object_diff_0, ObjectDiffResponse);
    deserialize_test!(v0_object_links_0, ObjectLinksResponse);
    deserialize_test!(v0_object_stat_0, ObjectStatResponse);
}
