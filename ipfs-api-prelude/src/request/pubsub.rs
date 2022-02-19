// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::request::ApiRequest;
use crate::serde::ser::SerializeStruct;
use multibase::{encode, Base};
use serde::{Serialize, Serializer};

pub struct PubsubLs;

impl_skip_serialize!(PubsubLs);

impl ApiRequest for PubsubLs {
    const PATH: &'static str = "/pubsub/ls";
}

pub struct PubsubPeers<'a> {
    pub topic: Option<&'a [u8]>,
}

impl<'a> Serialize for PubsubPeers<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("PubsubPeers", 1)?;

        if let Some(topic) = self.topic {
            let topic = encode(Base::Base64Url, topic);

            state.serialize_field("arg", &topic)?;
        } else {
            state.serialize_field("arg", &Option::<String>::None)?;
        }

        state.end()
    }
}

impl<'a> ApiRequest for PubsubPeers<'a> {
    const PATH: &'static str = "/pubsub/peers";
}

pub struct PubsubPub<'a> {
    pub topic: &'a [u8],
}

impl<'a> Serialize for PubsubPub<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("PubsubPub", 1)?;

        let topic = encode(Base::Base64Url, self.topic);

        state.serialize_field("arg", &topic)?;

        state.end()
    }
}

impl<'a> ApiRequest for PubsubPub<'a> {
    const PATH: &'static str = "/pubsub/pub";
}

pub struct PubsubSub<'a> {
    pub topic: &'a [u8],
}

impl<'a> Serialize for PubsubSub<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("PubsubSub", 1)?;

        let topic = encode(Base::Base64Url, self.topic);

        state.serialize_field("arg", &topic)?;

        state.end()
    }
}

impl<'a> ApiRequest for PubsubSub<'a> {
    const PATH: &'static str = "/pubsub/sub";
}
