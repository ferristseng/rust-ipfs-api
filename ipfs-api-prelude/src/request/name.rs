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
pub struct NamePublish<'a, 'b, 'c, 'd> {
    #[serde(rename = "arg")]
    pub path: &'a str,

    pub resolve: bool,

    pub lifetime: Option<&'b str>,

    pub ttl: Option<&'c str>,

    pub key: Option<&'d str>,
}

impl<'a, 'b, 'c, 'd> ApiRequest for NamePublish<'a, 'b, 'c, 'd> {
    const PATH: &'static str = "/name/publish";
}

#[derive(Serialize)]
pub struct NameResolve<'a> {
    #[serde(rename = "arg")]
    pub name: Option<&'a str>,

    pub recursive: bool,

    pub nocache: bool,
}

impl<'a> ApiRequest for NameResolve<'a> {
    const PATH: &'static str = "/name/resolve";
}
