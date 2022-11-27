// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::request::ApiRequest;
use serde::Serialize;

#[cfg_attr(feature = "with-builder", derive(TypedBuilder))]
#[derive(Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Add<'a> {
    /// Use trickle-dag format for dag generation.
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub trickle: Option<bool>,

    /// Only chunk and hash - do not write to disk.
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub only_hash: Option<bool>,

    /// Wrap files with a directory object.
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub wrap_with_directory: Option<bool>,

    /// Chunking algorithm, `size-[bytes]`, `rabin-[min]-[avg]-[max]` or `buzhash`.
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub chunker: Option<&'a str>,

    /// Pin this object when adding. Defaults to `true`.
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub pin: Option<bool>,

    /// Use raw blocks for leaf nodes. (experimental).
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub raw_leaves: Option<bool>,

    /// CID version. Defaults to 0 unless an option that depends on CIDv1 is passed.
    /// (experimental).
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub cid_version: Option<u32>,

    /// Hash function to use. Implies CIDv1 if not sha2-256. (experimental). Default:
    /// `sha2-256`.
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub hash: Option<&'a str>,

    /// Inline small blocks into CIDs. (experimental).
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub inline: Option<bool>,

    /// Maximum block size to inline. (experimental). Default: `32`.
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub inline_limit: Option<u32>,

    ///  Add reference to Files API (MFS) at the provided path
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub to_files: Option<&'a str>,
}

impl<'a> ApiRequest for Add<'a> {
    const PATH: &'static str = "/add";
}
