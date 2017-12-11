// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use request::ApiRequest;

#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PubsubLs;

impl_skip_serialize!(PubsubLs);

impl ApiRequest for PubsubLs {
    #[inline]
    fn path() -> &'static str {
        "/pubsub/ls"
    }
}


#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PubsubPeers<'a> {
    #[serde(rename = "arg")]
    pub topic: &'a Option<&'a str>,
}

impl<'a> ApiRequest for PubsubPeers<'a> {
    #[inline]
    fn path() -> &'static str {
        "/pubsub/peers"
    }
}


#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PubsubPub<'a> {
    #[serde(rename = "arg")]
    pub topic: &'a str,

    #[serde(rename = "arg")]
    pub payload: &'a str,
}

impl<'a> ApiRequest for PubsubPub<'a> {
    #[inline]
    fn path() -> &'static str {
        "/pubsub/pub"
    }
}


#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PubsubSub<'a> {
    #[serde(rename = "arg")]
    pub topic: &'a str,

    pub discover: bool,
}

impl<'a> ApiRequest for PubsubSub<'a> {
    #[inline]
    fn path() -> &'static str {
        "/pubsub/sub"
    }
}
