// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::response::serde;
use crate::serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PinAddResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub pins: Vec<String>,

    pub progress: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PinType {
    #[serde(rename = "Type")]
    pub typ: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PinLsResponse {
    #[serde(deserialize_with = "serde::deserialize_hashmap")]
    pub keys: HashMap<String, PinType>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PinRmResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub pins: Vec<String>,
}

#[cfg(test)]
mod tests {
    deserialize_test!(v0_pin_ls_0, PinLsResponse);
    deserialize_test!(v0_pin_add_0, PinAddResponse);
}
