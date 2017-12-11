// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use request::ApiRequest;

#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Ping<'a, 'b> {
    #[serde(rename = "arg")]
    pub peer: &'a str,

    pub count: &'b Option<i32>,
}

impl<'a, 'b> ApiRequest for Ping<'a, 'b> {
    const path: &'static str = "/ping";
}
