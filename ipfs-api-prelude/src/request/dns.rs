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
pub struct Dns<'a> {
    #[serde(rename = "arg")]
    pub link: &'a str,

    pub recursive: bool,
}

impl<'a> ApiRequest for Dns<'a> {
    const PATH: &'static str = "/dns";
}
