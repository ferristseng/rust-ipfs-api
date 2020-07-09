// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::request::ApiRequest;
use crate::serde::Serialize;

#[derive(Serialize)]
pub struct FilesCp<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,

    #[serde(rename = "arg")]
    pub dest: &'a str,

    pub flush: bool,
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

#[derive(Serialize)]
pub struct FilesLs<'a> {
    #[serde(rename = "arg")]
    pub path: Option<&'a str>,

    pub long: bool,

    #[serde(rename = "U")]
    pub unsorted: bool,
}

impl<'a> ApiRequest for FilesLs<'a> {
    const PATH: &'static str = "/files/ls";
}

#[derive(Serialize)]
pub struct FilesMkdir<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,

    pub parents: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<&'a str>,

    #[serde(rename = "cid-version")]
    pub cid_version: i32,

    pub flush: bool,
}

impl<'a> ApiRequest for FilesMkdir<'a> {
    const PATH: &'static str = "/files/mkdir";
}

#[derive(Serialize)]
pub struct FilesMv<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,

    #[serde(rename = "arg")]
    pub dest: &'a str,

    pub flush: bool,
}

impl<'a> ApiRequest for FilesMv<'a> {
    const PATH: &'static str = "/files/mv";
}

#[derive(Serialize)]
pub struct FilesRead<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,

    pub offset: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

impl<'a> ApiRequest for FilesRead<'a> {
    const PATH: &'static str = "/files/read";
}

#[derive(Serialize)]
pub struct FilesRm<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,

    pub recursive: bool,

    pub flush: bool,
}

impl<'a> ApiRequest for FilesRm<'a> {
    const PATH: &'static str = "/files/rm";
}

#[derive(Serialize)]
pub struct FilesStat<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,

    #[serde(rename = "with-local")]
    pub with_local: bool,
}

impl<'a> ApiRequest for FilesStat<'a> {
    const PATH: &'static str = "/files/stat";
}

#[derive(Serialize)]
pub struct FilesWrite<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,

    pub create: bool,

    pub truncate: bool,

    pub parents: bool,

    pub offset: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,

    #[serde(rename = "raw-leaves")]
    pub raw_leaves: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<&'a str>,

    #[serde(rename = "cid-version")]
    pub cid_version: i32,

    pub flush: bool,
}

impl<'a> ApiRequest for FilesWrite<'a> {
    const PATH: &'static str = "/files/write";
}

#[derive(Serialize)]
pub struct FilesChcid<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<&'a str>,

    #[serde(rename = "cid-version")]
    pub cid_version: i32,

    pub flush: bool,
}

impl<'a> ApiRequest for FilesChcid<'a> {
    const PATH: &'static str = "/files/chcid";
}
