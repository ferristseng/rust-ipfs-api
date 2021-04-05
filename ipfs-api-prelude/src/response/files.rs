// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::response::serde;
use crate::serde::Deserialize;

pub type FilesCpResponse = ();

pub type FilesFlushResponse = ();

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FilesEntry {
    pub name: String,

    // This is a protocol buffer enum type defined in
    // https://github.com/ipfs/go-ipfs/blob/master/unixfs/pb/unixfs.proto ...
    // So it might be some other type than u64, but certainly shouldn't be *bigger* than u64.
    #[serde(rename = "Type")]
    pub typ: u64,
    pub size: u64,
    pub hash: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FilesLsResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub entries: Vec<FilesEntry>,
}

pub type FilesMkdirResponse = ();

pub type FilesMvResponse = ();

pub type FilesRmResponse = ();

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FilesStatResponse {
    pub hash: String,
    pub size: u64,
    pub cumulative_size: u64,
    pub blocks: u64,

    #[serde(rename = "Type")]
    pub typ: String,

    #[serde(default)]
    pub size_local: Option<u64>,
    #[serde(default)]
    pub local: Option<bool>,
}

pub type FilesWriteResponse = ();

pub type FilesChcidResponse = ();

#[cfg(test)]
mod tests {
    deserialize_test!(v0_files_ls_0, FilesLsResponse);
    deserialize_test!(v0_files_stat_0, FilesStatResponse);
}
