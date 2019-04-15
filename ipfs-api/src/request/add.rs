// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use http::Method;
use request::ApiRequest;

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Add<'a, 'b> {
    pub recursive: Option<bool>,
    pub progress: Option<bool>,
    pub trickle: Option<bool>,
    pub only_hash: Option<bool>,
    pub chunker: Option<&'a str>,
    pub pin: Option<bool>,
    pub raw_leaves: Option<bool>,
    pub fscache: Option<bool>,
    pub cid_version: Option<isize>,
    pub hash: Option<&'b str>,
    pub inline: Option<bool>,
    pub inline_limit: Option<isize>
}

impl<'a,'b> Default for Add<'a,'b> {
    fn default() -> Self {
        Self {
            recursive: None,
            progress: None,
            trickle: None,
            only_hash: None,
            chunker: None,
            pin: None,
            raw_leaves: None,
            fscache: None,
            cid_version: None,
            hash: None,
            inline: None,
            inline_limit: None,
        }
    }
}

impl<'a, 'b> ApiRequest for Add<'a,'b> {
    const PATH: &'static str = "/add";

    const METHOD: &'static Method = &Method::POST;
}
