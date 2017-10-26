// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use response::serde;
use std::collections::HashMap;


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RepoFsckResponse {
    pub message: String,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RepoGcResponse {
    #[serde(deserialize_with = "serde::deserialize_hashmap")]
    pub key: HashMap<String, String>,
    pub error: Option<String>,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RepoStatResponse {
    pub num_objects: u64,
    pub repo_size: u64,
    pub repo_path: String,
    pub version: String,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RepoVerifyResponse {
    pub message: String,
    pub progress: isize,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RepoVersionResponse {
    pub version: String,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_repo_gc_0, RepoGcResponse);
    deserialize_test!(v0_repo_stat_0, RepoStatResponse);
    deserialize_test!(v0_repo_verify_0, RepoVerifyResponse);
    deserialize_test!(v0_repo_verify_1, RepoVerifyResponse);
    deserialize_test!(v0_repo_version_0, RepoVersionResponse);
}
