// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use request::ApiRequest;


pub struct FilestoreDups;

impl_skip_serialize!(FilestoreDups);

impl ApiRequest for FilestoreDups {
    #[inline]
    fn path() -> &'static str {
        "/filestore/dups"
    }
}


#[derive(Serialize)]
pub struct FilestoreLs<'a> {
    #[serde(rename = "arg")]
    pub cid: Option<&'a str>,
}

impl<'a> ApiRequest for FilestoreLs<'a> {
    #[inline]
    fn path() -> &'static str {
        "/filestore/ls"
    }
}


#[derive(Serialize)]
pub struct FilestoreVerify<'a> {
    #[serde(rename = "arg")]
    pub cid: Option<&'a str>,
}

impl<'a> ApiRequest for FilestoreVerify<'a> {
    #[inline]
    fn path() -> &'static str {
        "/filestore/verify"
    }
}
