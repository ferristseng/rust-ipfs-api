// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use request::ApiRequest;

#[derive(Serialize)]
pub struct Ls<'a> {
    #[serde(rename = "arg")]
    pub path: Option<&'a str>,
}

impl<'a> ApiRequest for Ls<'a> {
    const PATH: &'static str = "/ls";
}

#[cfg(test)]
mod tests {
    use super::Ls;

    serialize_url_test!(test_serializes_0, Ls { path: Some("test") }, "arg=test");
    serialize_url_test!(test_serializes_1, Ls { path: None }, "");
}
