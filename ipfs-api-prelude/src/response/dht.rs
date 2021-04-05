// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::response::serde;
use crate::serde::{
    de::{Deserializer, Error},
    Deserialize,
};

/// See
/// [libp2p](https://github.com/libp2p/go-libp2p-routing/blob/master/notifications/query.go#L16).
///
#[derive(Debug)]
pub enum DhtType {
    SendingQuery,
    PeerResponse,
    FinalPeer,
    QueryError,
    Provider,
    Value,
    AddingPeer,
    DialingPeer,
}

impl<'de> Deserialize<'de> for DhtType {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match deserializer.deserialize_i64(serde::IntegerVisitor)? {
            0 => Ok(DhtType::SendingQuery),
            1 => Ok(DhtType::PeerResponse),
            2 => Ok(DhtType::FinalPeer),
            3 => Ok(DhtType::QueryError),
            4 => Ok(DhtType::Provider),
            5 => Ok(DhtType::Value),
            6 => Ok(DhtType::AddingPeer),
            7 => Ok(DhtType::DialingPeer),
            i => Err(D::Error::custom(format!("unknown dht type '{}'", i))),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DhtPeerResponse {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub addrs: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DhtMessage {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "Type")]
    pub typ: DhtType,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub responses: Vec<DhtPeerResponse>,

    pub extra: String,
}

pub type DhtFindPeerResponse = DhtMessage;

pub type DhtFindProvsResponse = DhtMessage;

pub type DhtGetResponse = DhtMessage;

pub type DhtProvideResponse = DhtMessage;

pub type DhtPutResponse = DhtMessage;

pub type DhtQueryResponse = DhtMessage;
