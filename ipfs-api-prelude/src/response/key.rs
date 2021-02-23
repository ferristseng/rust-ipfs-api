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
pub struct KeyPair {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct KeyPairList {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub keys: Vec<KeyPair>,
}

pub type KeyGenResponse = KeyPair;

pub type KeyListResponse = KeyPairList;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct KeyRenameResponse {
    pub was: String,
    pub now: String,
    pub id: String,
    pub overwrite: bool,
}

pub type KeyRmResponse = KeyPairList;

#[cfg(test)]
mod tests {
    deserialize_test!(v0_key_gen_0, KeyGenResponse);
    deserialize_test!(v0_key_list_0, KeyListResponse);
    deserialize_test!(v0_key_rename_0, KeyRenameResponse);
}
