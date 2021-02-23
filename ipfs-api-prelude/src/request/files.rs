// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::request::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Default)]
pub struct FilesCp<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,

    #[serde(rename = "arg")]
    pub dest: &'a str,

    pub flush: Option<bool>,
}

impl<'a> ApiRequest for FilesCp<'a> {
    const PATH: &'static str = "/files/cp";
}

#[derive(Serialize)]
pub struct FilesFlush<'a> {
    #[serde(rename = "arg")]
    pub path: Option<&'a str>,
}

impl<'a> ApiRequest for FilesFlush<'a> {
    const PATH: &'static str = "/files/flush";
}

#[cfg_attr(feature = "with-builder", derive(TypedBuilder))]
#[derive(Serialize, Default)]
pub struct FilesLs<'a> {
    #[serde(rename = "arg")]
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub path: Option<&'a str>,

    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub long: Option<bool>,

    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    #[serde(rename = "U")]
    pub unsorted: Option<bool>,
}

impl<'a> ApiRequest for FilesLs<'a> {
    const PATH: &'static str = "/files/ls";
}

#[cfg_attr(feature = "with-builder", derive(TypedBuilder))]
#[derive(Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct FilesMkdir<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,

    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub parents: Option<bool>,

    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub hash: Option<&'a str>,

    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub cid_version: Option<i32>,

    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub flush: Option<bool>,
}

impl<'a> ApiRequest for FilesMkdir<'a> {
    const PATH: &'static str = "/files/mkdir";
}

#[derive(Serialize, Default)]
pub struct FilesMv<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,

    #[serde(rename = "arg")]
    pub dest: &'a str,

    pub flush: Option<bool>,
}

impl<'a> ApiRequest for FilesMv<'a> {
    const PATH: &'static str = "/files/mv";
}

#[cfg_attr(feature = "with-builder", derive(TypedBuilder))]
#[derive(Serialize, Default)]
pub struct FilesRead<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,

    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub offset: Option<i64>,

    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub count: Option<i64>,
}

impl<'a> ApiRequest for FilesRead<'a> {
    const PATH: &'static str = "/files/read";
}

#[cfg_attr(feature = "with-builder", derive(TypedBuilder))]
#[derive(Serialize, Default)]
pub struct FilesRm<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,

    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub recursive: Option<bool>,

    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub flush: Option<bool>,
}

impl<'a> ApiRequest for FilesRm<'a> {
    const PATH: &'static str = "/files/rm";
}

#[derive(Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct FilesStat<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,

    pub with_local: Option<bool>,
}

impl<'a> ApiRequest for FilesStat<'a> {
    const PATH: &'static str = "/files/stat";
}

#[cfg_attr(feature = "with-builder", derive(TypedBuilder))]
#[derive(Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct FilesWrite<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,

    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub create: Option<bool>,

    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub truncate: Option<bool>,

    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub parents: Option<bool>,

    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub offset: Option<i64>,

    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub count: Option<i64>,

    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub raw_leaves: Option<bool>,

    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub hash: Option<&'a str>,

    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub cid_version: Option<i32>,

    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub flush: Option<bool>,
}

impl<'a> ApiRequest for FilesWrite<'a> {
    const PATH: &'static str = "/files/write";
}

#[cfg_attr(feature = "with-builder", derive(TypedBuilder))]
#[derive(Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct FilesChcid<'a> {
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    #[serde(rename = "arg")]
    pub path: Option<&'a str>,

    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub hash: Option<&'a str>,

    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub cid_version: Option<i32>,

    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub flush: Option<bool>,
}

impl<'a> ApiRequest for FilesChcid<'a> {
    const PATH: &'static str = "/files/chcid";
}
