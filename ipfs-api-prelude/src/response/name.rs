// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NamePublishResponse {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NameResolveResponse {
    pub path: String,
}

#[cfg(test)]
mod tests {
    deserialize_test!(v0_name_resolve_0, NameResolveResponse);
}
