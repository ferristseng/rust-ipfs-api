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
pub struct PinAdd<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,

    pub recursive: Option<bool>,
    pub progress: bool,
}

impl<'a> ApiRequest for PinAdd<'a> {
    const PATH: &'static str = "/pin/add";
}

#[derive(Serialize)]
pub struct PinLs<'a> {
    #[serde(rename = "arg")]
    pub key: Option<&'a str>,

    #[serde(rename = "type")]
    pub typ: Option<&'a str>,
}

impl<'a> ApiRequest for PinLs<'a> {
    const PATH: &'static str = "/pin/ls";
}

#[derive(Serialize)]
pub struct PinRm<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,

    pub recursive: bool,
}

impl<'a> ApiRequest for PinRm<'a> {
    const PATH: &'static str = "/pin/rm";
}
