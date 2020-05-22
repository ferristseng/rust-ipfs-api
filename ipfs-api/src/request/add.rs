// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::request::ApiRequest;
use derive_builder::Builder;
use serde::Serialize;

#[derive(Serialize, Builder, Default)]
#[serde(rename_all = "kebab-case")]
#[builder(pattern = "owned", setter(strip_option))]
pub struct Add<'a> {
    /// Use trickle-dag format for dag generation.
    pub trickle: Option<bool>,
    /// Only chunk and hash - do not write to disk.
    pub only_hash: Option<bool>,
    /// Wrap files with a directory object.
    pub wrap_with_directory: Option<bool>,
    /// Chunking algorithm, `size-[bytes]`, `rabin-[min]-[avg]-[max]` or `buzhash`.
    pub chunker: Option<&'a str>,
    /// Pin this object when adding. Defaults to `true`.
    pub pin: Option<bool>,
    /// Use raw blocks for leaf nodes. (experimental).
    pub raw_leaves: Option<bool>,
    /// CID version. Defaults to 0 unless an option that depends on CIDv1 is passed.
    /// (experimental).
    pub cid_version: Option<u32>,
    /// Hash function to use. Implies CIDv1 if not sha2-256. (experimental). Default:
    /// `sha2-256`.
    pub hash: Option<&'a str>,
    /// Inline small blocks into CIDs. (experimental).
    pub inline: Option<bool>,
    /// Maximum block size to inline. (experimental). Default: `32`.
    pub inline_limit: Option<u32>,
}

impl<'a> ApiRequest for Add<'a> {
    const PATH: &'static str = "/add";
}
