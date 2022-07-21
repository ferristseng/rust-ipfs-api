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
pub struct Cat<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,
}

#[derive(Serialize)]
pub struct CatBytes<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,

    pub offset: i64,
    pub length: i64,
}

impl<'a> ApiRequest for Cat<'a>{
    const PATH: &'static str = "/cat";
}

impl<'a> ApiRequest for CatBytes<'a> {
    const PATH: &'static str = "/cat";
}
