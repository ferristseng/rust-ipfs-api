// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use request::ApiRequest;


#[derive(Serialize)]
pub struct FilesCp<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,

    #[serde(rename = "arg")]
    pub dest: &'a str,
}

impl<'a> ApiRequest for FilesCp<'a> {
    #[inline]
    fn path() -> &'static str {
        "/files/cp"
    }
}


#[derive(Serialize)]
pub struct FilesFlush<'a> {
    #[serde(rename = "arg")]
    pub path: &'a Option<&'a str>,
}

impl<'a> ApiRequest for FilesFlush<'a> {
    #[inline]
    fn path() -> &'static str {
        "/files/flush"
    }
}


#[derive(Serialize)]
pub struct FilesLs<'a> {
    #[serde(rename = "arg")]
    pub path: &'a Option<&'a str>,
}

impl<'a> ApiRequest for FilesLs<'a> {
    #[inline]
    fn path() -> &'static str {
        "/files/ls"
    }
}


#[derive(Serialize)]
pub struct FilesMkdir<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,

    pub parents: bool,
}

impl<'a> ApiRequest for FilesMkdir<'a> {
    #[inline]
    fn path() -> &'static str {
        "/files/mkdir"
    }
}


#[derive(Serialize)]
pub struct FilesMv<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,

    #[serde(rename = "arg")]
    pub dest: &'a str,
}

impl<'a> ApiRequest for FilesMv<'a> {
    #[inline]
    fn path() -> &'static str {
        "/files/mv"
    }
}


#[derive(Serialize)]
pub struct FilesRead<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,
}

impl<'a> ApiRequest for FilesRead<'a> {
    #[inline]
    fn path() -> &'static str {
        "/files/read"
    }
}


#[derive(Serialize)]
pub struct FilesRm<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,

    pub recursive: bool,
}

impl<'a> ApiRequest for FilesRm<'a> {
    #[inline]
    fn path() -> &'static str {
        "/files/rm"
    }
}


#[derive(Serialize)]
pub struct FilesStat<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,
}

impl<'a> ApiRequest for FilesStat<'a> {
    #[inline]
    fn path() -> &'static str {
        "/files/stat"
    }
}


#[derive(Serialize)]
pub struct FilesWrite<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,

    pub create: bool,

    pub truncate: bool,
}

impl<'a> ApiRequest for FilesWrite<'a> {
    #[inline]
    fn path() -> &'static str {
        "/files/write"
    }
}
