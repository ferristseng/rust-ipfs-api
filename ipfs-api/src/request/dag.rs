// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use hyper::Method;
use request::ApiRequest;

#[derive(Serialize)]
pub struct DagGet<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,
}

impl<'a> ApiRequest for DagGet<'a> {
    const PATH: &'static str = "/dag/get";
}

pub struct DagPut;

impl_skip_serialize!(DagPut);

impl ApiRequest for DagPut {
    const PATH: &'static str = "/dag/put";

    const METHOD: &'static Method = &Method::Post;
}
