// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use request::ApiRequest;


#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ObjectDiff<'a> {
    #[serde(rename = "arg")]
    pub key0: &'a str,

    #[serde(rename = "arg")]
    pub key1: &'a str,
}

impl<'a> ApiRequest for ObjectDiff<'a> {
    const PATH: &'static str = "/object/diff";
}


#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ObjectGet<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,
}

impl<'a> ApiRequest for ObjectGet<'a> {
    const PATH: &'static str = "/object/get";
}


#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ObjectLinks<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,
}

impl<'a> ApiRequest for ObjectLinks<'a> {
        const PATH: &'static str = "/object/links";
}


#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ObjectStat<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,
}

impl<'a> ApiRequest for ObjectStat<'a> {
    const PATH: &'static str = "/object/stat";
}


#[cfg(test)]
mod tests {
    use super::ObjectDiff;

    serialize_url_test!(
        test_serializes_0,
        ObjectDiff {
            key0: "test",
            key1: "test2",
        },
        "arg=test&arg=test2"
    );
}
