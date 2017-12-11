// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use request::ApiRequest;
use serde::ser::{Serialize, Serializer};


#[derive(Copy, Clone)]
pub enum KeyType {
    Rsa,
    Ed25519,
}

impl Serialize for KeyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            &KeyType::Rsa => "rsa",
            &KeyType::Ed25519 => "ed25519",
        };

        serializer.serialize_str(s)
    }
}


#[derive(Serialize)]
pub struct KeyGen<'a> {
    #[serde(rename = "arg")]
    pub name: &'a str,

    #[serde(rename = "type")]
    pub kind: KeyType,

    pub size: Option<i32>,
}

impl<'a> ApiRequest for KeyGen<'a> {
    #[inline]
    fn path() -> &'static str {
        "/key/gen"
    }
}


pub struct KeyList;

impl_skip_serialize!(KeyList);

impl ApiRequest for KeyList {
    #[inline]
    fn path() -> &'static str {
        "/key/list"
    }
}
