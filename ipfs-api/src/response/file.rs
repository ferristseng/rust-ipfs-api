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
pub struct IpfsDetailedFile {
    pub hash: String,
    pub size: u64,

    #[serde(rename = "Type")]
    pub typ: String,

    #[serde(default)]
    pub links: Vec<IpfsHeader>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FileLsResponse {
    #[serde(deserialize_with = "serde::deserialize_hashmap")]
    pub arguments: HashMap<String, String>,

    #[serde(deserialize_with = "serde::deserialize_hashmap")]
    pub objects: HashMap<String, IpfsDetailedFile>,
}

#[cfg(test)]
mod tests {
    deserialize_test!(v0_file_ls_0, FileLsResponse);
    deserialize_test!(v0_file_ls_1, FileLsResponse);
}
