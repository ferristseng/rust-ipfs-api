// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::request::ApiRequest;
use serde::Serialize;

#[cfg_attr(feature = "with-builder", derive(TypedBuilder))]
#[derive(Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Ls<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,
    /// Resolve linked objects to find out their types. Default: `true`
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub resolve_type: Option<bool>,
    /// Resolve linked objects to find out their file size. Default: `true`
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub size: Option<bool>,
    /// Enable experimental streaming of directory entries as they are traversed.
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub stream: Option<bool>,
}

impl<'a> ApiRequest for Ls<'a> {
    const PATH: &'static str = "/ls";
}

#[cfg(test)]
mod tests {
    use super::Ls;

    serialize_url_test!(
        test_serializes_0,
        Ls {
            path: "test",
            ..Default::default()
        },
        "arg=test"
    );
    serialize_url_test!(
        test_serializes_1,
        Ls {
            path: "asdf",
            resolve_type: Some(true),
            size: Some(true),
            stream: Some(false)
        },
        "arg=asdf&resolve-type=true&size=true&stream=false"
    );
}
