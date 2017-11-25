// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

pub type BlockGetResponse = Vec<u8>;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockPutResponse {
    pub key: String,
    pub size: usize,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockRmResponse {
    pub hash: String,
    pub error: Option<String>,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockStatResponse {
    pub key: String,
    pub size: usize,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_block_stat_0, BlockStatResponse);
}
