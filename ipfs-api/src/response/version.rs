// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VersionResponse {
    pub version: String,
    pub commit: String,
    pub repo: String,
    pub system: String,
    pub golang: String,
}

#[cfg(test)]
mod tests {
    deserialize_test!(v0_version_0, VersionResponse);
}
