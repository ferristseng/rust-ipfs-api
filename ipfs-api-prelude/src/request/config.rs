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
pub struct Config<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,

    #[serde(rename = "arg")]
    pub value: Option<&'a str>,

    #[serde(rename = "bool")]
    pub boolean: Option<bool>,

    #[serde(rename = "json")]
    pub stringified_json: Option<bool>,
}

impl<'a> ApiRequest for Config<'a> {
    const PATH: &'static str = "/config";
}

pub struct ConfigEdit;

impl_skip_serialize!(ConfigEdit);

impl ApiRequest for ConfigEdit {
    const PATH: &'static str = "/config/edit";
}

pub struct ConfigReplace;

impl_skip_serialize!(ConfigReplace);

impl ApiRequest for ConfigReplace {
    const PATH: &'static str = "/config/replace";
}

pub struct ConfigShow;

impl_skip_serialize!(ConfigShow);

impl ApiRequest for ConfigShow {
    const PATH: &'static str = "/config/show";
}
