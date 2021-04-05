// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::response::serde;
use crate::serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IpfsFile {
    pub hash: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub links: Vec<IpfsFileHeader>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IpfsFileHeader {
    pub name: String,
    pub hash: String,
    pub size: u64,

    #[serde(rename = "Type")]
    pub typ: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LsResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub objects: Vec<IpfsFile>,
}

#[cfg(test)]
mod tests {
    deserialize_test!(v0_ls_0, LsResponse);
    deserialize_test!(v0_ls_1, LsResponse);
}
