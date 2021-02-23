// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::request::ApiRequest;
use serde::Serialize;

#[derive(Serialize)]
pub struct BitswapLedger<'a> {
    #[serde(rename = "arg")]
    pub peer: &'a str,
}

impl<'a> ApiRequest for BitswapLedger<'a> {
    const PATH: &'static str = "/bitswap/ledger";
}

pub struct BitswapReprovide;

impl_skip_serialize!(BitswapReprovide);

impl ApiRequest for BitswapReprovide {
    const PATH: &'static str = "/bitswap/reprovide";
}

pub struct BitswapStat;

impl_skip_serialize!(BitswapStat);

impl ApiRequest for BitswapStat {
    const PATH: &'static str = "/bitswap/stat";
}

#[derive(Serialize)]
pub struct BitswapUnwant<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,
}

impl<'a> ApiRequest for BitswapUnwant<'a> {
    const PATH: &'static str = "/bitswap/stat";
}

#[derive(Serialize)]
pub struct BitswapWantlist<'a> {
    pub peer: Option<&'a str>,
}

impl<'a> ApiRequest for BitswapWantlist<'a> {
    const PATH: &'static str = "/bitswap/wantlist";
}
