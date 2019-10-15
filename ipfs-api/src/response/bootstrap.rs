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
pub struct BootstrapAddDefaultResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub peers: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BootstrapListResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub peers: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BootstrapRmAllResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub peers: Vec<String>,
}

#[cfg(test)]
mod tests {
    deserialize_test!(v0_bootstrap_list_0, BootstrapListResponse);
}
