// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::request::ApiRequest;
use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub struct ObjectData<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,
}

impl<'a> ApiRequest for ObjectData<'a> {
    const PATH: &'static str = "/object/data";
}

#[derive(Serialize)]
pub struct ObjectDiff<'a> {
    #[serde(rename = "arg")]
    pub key0: &'a str,

    #[serde(rename = "arg")]
    pub key1: &'a str,
}

impl<'a> ApiRequest for ObjectDiff<'a> {
    const PATH: &'static str = "/object/diff";
}

#[derive(Serialize)]
pub struct ObjectGet<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,
}

impl<'a> ApiRequest for ObjectGet<'a> {
    const PATH: &'static str = "/object/get";
}

#[derive(Serialize)]
pub struct ObjectLinks<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,
}

impl<'a> ApiRequest for ObjectLinks<'a> {
    const PATH: &'static str = "/object/links";
}

#[derive(Copy, Clone)]
pub enum ObjectTemplate {
    UnixFsDir,
}

impl Serialize for ObjectTemplate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            ObjectTemplate::UnixFsDir => "unixfs-dir",
        };

        serializer.serialize_str(s)
    }
}

#[derive(Serialize)]
pub struct ObjectNew {
    #[serde(rename = "arg")]
    pub template: Option<ObjectTemplate>,
}

impl ApiRequest for ObjectNew {
    const PATH: &'static str = "/object/new";
}

#[derive(Serialize)]
pub struct ObjectPatchAddLink<'a> {
    #[serde(rename = "arg")]
    pub folder: &'a str,

    #[serde(rename = "arg")]
    pub name: &'a str,

    #[serde(rename = "arg")]
    pub key: &'a str,

    #[serde(rename = "create")]
    pub create: bool,
}

impl<'a> ApiRequest for ObjectPatchAddLink<'a> {
    const PATH: &'static str = "/object/patch/add-link";
}

#[derive(Serialize)]
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
