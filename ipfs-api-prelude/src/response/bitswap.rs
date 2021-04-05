// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::response::serde;
use crate::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BitswapLedgerResponse {
    pub peer: String,
    pub value: f64,
    pub sent: u64,
    pub recv: u64,
    pub exchanged: u64,
}

pub type BitswapReprovideResponse = ();

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BitswapStatResponse {
    pub provide_buf_len: i32,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub wantlist: Vec<String>,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub peers: Vec<String>,

    pub blocks_received: u64,
    pub data_received: u64,
    pub blocks_sent: u64,
    pub data_sent: u64,
    pub dup_blks_received: u64,
    pub dup_data_received: u64,
}

pub type BitswapUnwantResponse = ();

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BitswapWantlistResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub keys: Vec<String>,
}

#[cfg(test)]
mod tests {
    deserialize_test!(v0_bitswap_stat_0, BitswapStatResponse);
}
