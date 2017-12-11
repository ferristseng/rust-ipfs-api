// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use request::ApiRequest;


#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BlockGet<'a> {
    #[serde(rename = "arg")]
    pub hash: &'a str,
}

impl<'a> ApiRequest for BlockGet<'a> {
    const path: &'static str = "/block/get";
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BlockPut;

impl_skip_serialize!(BlockPut);

impl ApiRequest for BlockPut {
    const path: &'static str = "/block/put";
}


#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BlockRm<'a> {
    #[serde(rename = "arg")]
    pub hash: &'a str,
}

impl<'a> ApiRequest for BlockRm<'a> {
    const path: &'static str = "/block/rm";
}


#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BlockStat<'a> {
    #[serde(rename = "arg")]
    pub hash: &'a str,
}

impl<'a> ApiRequest for BlockStat<'a> {
    const path: &'static str = "/block/stat";
}
