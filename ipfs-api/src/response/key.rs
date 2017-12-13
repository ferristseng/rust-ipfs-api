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
pub struct KeyGenResponse {
    pub name: String,
    pub id: String,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct KeyListResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub keys: Vec<KeyGenResponse>,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_key_gen_0, KeyGenResponse);
    deserialize_test!(v0_key_list_0, KeyListResponse);
}
