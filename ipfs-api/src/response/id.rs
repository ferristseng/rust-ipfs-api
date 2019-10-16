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
pub struct IdResponse {
    #[serde(rename = "ID")]
    pub id: String,

    pub public_key: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub addresses: Vec<String>,

    pub agent_version: String,
    pub protocol_version: String,
}

#[cfg(test)]
mod tests {
    deserialize_test!(v0_id_0, IdResponse);
}
