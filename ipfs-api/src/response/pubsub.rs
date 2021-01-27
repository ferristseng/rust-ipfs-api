// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::response::serde;
use crate::serde::{Deserialize, Deserializer};

use std::convert::TryFrom;
use std::convert::TryInto;
use std::str::FromStr;

use cid::Cid;
use multibase::Base;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PubsubLsResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub strings: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PubsubPeersResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub strings: Vec<String>,
}

pub type PubsubPubResponse = ();

#[derive(Debug, Deserialize)]
pub struct PubsubSubResponse {
    //#[serde(deserialize_with = "deserialize_from_field")]
    pub from: Option<String>,

    #[serde(deserialize_with = "deserialize_data_field")]
    pub data: Option<String>,

    #[serde(deserialize_with = "deserialize_seqno_field")]
    pub seqno: Option<usize>,

    #[serde(rename = "topicIDs")]
    pub topic_ids: Option<Vec<String>>,

    #[serde(rename = "XXX_unrecognized")]
    pub unrecognized: Option<Vec<u8>>,
}

/* fn deserialize_from_field<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let from: Option<String> = Deserialize::deserialize(deserializer)?;

    let from = match from {
        Some(from) => from,
        None => return Ok(None),
    };

    let from = Base::decode(&Base::Base64Pad, from).expect("Multibase decoding failed");

    //let from = String::from_utf8(from).expect("Invalid UTF-8");

    //let from = Cid::try_from(from).expect("Deserialize string to CID failed");

    Ok(Some(from))
} */

fn deserialize_data_field<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let data: Option<&str> = Deserialize::deserialize(deserializer)?;

    let data = match data {
        Some(data) => data,
        None => return Ok(None),
    };

    let data = Base::decode(&Base::Base64Pad, data).expect("Multibase decoding failed");

    let data = String::from_utf8(data).expect("Invalid UTF-8");

    Ok(Some(data))
}

fn deserialize_seqno_field<'de, D>(deserializer: D) -> Result<Option<usize>, D::Error>
where
    D: Deserializer<'de>,
{
    let seqno: Option<&str> = Deserialize::deserialize(deserializer)?;

    let seqno = match seqno {
        Some(seqno) => seqno,
        None => return Ok(None),
    };

    let seqno = Base::decode(&Base::Base64Pad, seqno).expect("Multibase decoding failed");

    let seqno = seqno.try_into().expect("Not 64 bits");

    let seqno = usize::from_be_bytes(seqno);

    Ok(Some(seqno))
}

#[cfg(test)]
mod tests {
    deserialize_test!(v0_pubsub_ls_0, PubsubLsResponse);
    deserialize_test!(v0_pubsub_ls_1, PubsubLsResponse);
    deserialize_test!(v0_pubsub_peers_0, PubsubPeersResponse);
    deserialize_test!(v0_pubsub_sub_0, PubsubSubResponse);
    deserialize_test!(v0_pubsub_sub_1, PubsubSubResponse);
}
