// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use request::ApiRequest;


#[derive(Serialize)]
pub struct PinLs<'a> {
    #[serde(rename = "arg")]
    pub key: &'a Option<&'a str>,

    #[serde(rename = "type")]
    pub typ: &'a Option<&'a str>,
}

impl<'a> ApiRequest for PinLs<'a> {
    #[inline]
    fn path() -> &'static str {
        "/pin/ls"
    }
}


#[derive(Serialize)]
pub struct PinRm<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,

    pub recursive: bool,
}

impl<'a> ApiRequest for PinRm<'a> {
    #[inline]
    fn path() -> &'static str {
        "/pin/rm"
    }
}
