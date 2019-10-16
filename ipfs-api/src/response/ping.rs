// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PingResponse {
    pub success: bool,
    pub time: i64,
    pub text: String,
}

#[cfg(test)]
mod tests {
    deserialize_test!(v0_ping_0, PingResponse);
    deserialize_test!(v0_ping_1, PingResponse);
    deserialize_test!(v0_ping_2, PingResponse);
}
