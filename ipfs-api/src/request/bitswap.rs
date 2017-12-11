// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use request::ApiRequest;


#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BitswapLedger<'a> {
    #[serde(rename = "arg")]
    pub peer: &'a str,
}

impl<'a> ApiRequest for BitswapLedger<'a> {
    const path: &'static str = "/bitswap/ledger";
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BitswapStat;

impl_skip_serialize!(BitswapStat);

impl ApiRequest for BitswapStat {
    const path: &'static str = "/bitswap/stat";
}


#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BitswapUnwant<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,
}

impl<'a> ApiRequest for BitswapUnwant<'a> {
    const path: &'static str = "/bitswap/stat";
}


#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BitswapWantlist<'a> {
    pub peer: Option<&'a str>,
}

impl<'a> ApiRequest for BitswapWantlist<'a> {
    const path: &'static str = "/bitswap/wantlist";
}
