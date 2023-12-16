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
#[derive(Default, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Id<'a> {
    /// Peer.ID of node to look up.
    #[serde(rename = "arg")]
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub peer: Option<&'a str>,

    /// Ignored by go-ipfs in it's REST API. Always returns in JSON. Retained for compatibility.
    #[serde(skip)]
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub format: Option<IdFormat>,

    /// Encoding used for peer IDs: Can either be a multibase encoded CID or a base58btc encoded multihash. Takes {b58mh|base36|k|base32|b...}. Default: b58mh.
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub peerid_base: Option<&'a str>,
}

impl<'a> ApiRequest for Id<'a> {
    const PATH: &'static str = "/id";
}

pub enum IdFormat {}
