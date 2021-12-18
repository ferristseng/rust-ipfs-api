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
pub struct BlockGet<'a> {
    #[serde(rename = "arg")]
    pub hash: &'a str,
}

impl<'a> ApiRequest for BlockGet<'a> {
    const PATH: &'static str = "/block/get";
}

#[cfg_attr(feature = "with-builder", derive(TypedBuilder))]
#[derive(Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct BlockPut<'a> {
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub format: Option<&'a str>,
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub mhtype: Option<&'a str>,
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub mhlen: Option<u32>,
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub pin: Option<bool>,
}

impl<'a> ApiRequest for BlockPut<'a> {
    const PATH: &'static str = "/block/put";
}

#[derive(Serialize)]
pub struct BlockRm<'a> {
    #[serde(rename = "arg")]
    pub hash: &'a str,
}

impl<'a> ApiRequest for BlockRm<'a> {
    const PATH: &'static str = "/block/rm";
}

#[derive(Serialize)]
pub struct BlockStat<'a> {
    #[serde(rename = "arg")]
    pub hash: &'a str,
}

impl<'a> ApiRequest for BlockStat<'a> {
    const PATH: &'static str = "/block/stat";
}
