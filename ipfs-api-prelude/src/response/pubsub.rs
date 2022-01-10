// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::response::serde;
use crate::serde::{Deserialize, Deserializer};

use multibase::decode;
use std::convert::TryInto;

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
    pub from: String,

    #[serde(deserialize_with = "deserialize_data_field")]
    pub data: Vec<u8>,

    #[serde(deserialize_with = "deserialize_seqno_field")]
    pub seqno: u64,

    #[serde(rename = "topicIDs", deserialize_with = "deserialize_topic_field")]
    pub topic_ids: Vec<String>,
}

fn deserialize_data_field<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let data: &str = Deserialize::deserialize(deserializer)?;

    let (_, data) = decode(data).expect("Multibase Decoding");

    Ok(data)
}

fn deserialize_seqno_field<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let seqno: &str = Deserialize::deserialize(deserializer)?;

    let (_, seqno) = decode(seqno).expect("Multibase Decoding");

    let seqno = seqno.try_into().expect("64 bits Type");

    let seqno = u64::from_be_bytes(seqno);

    Ok(seqno)
}

fn deserialize_topic_field<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let topic_ids: Vec<&str> = Deserialize::deserialize(deserializer)?;

    let topic_ids = topic_ids
        .into_iter()
        .map(|topic_id| {
            let (_, topic_id) = decode(topic_id).expect("Multibase Decoding");

            String::from_utf8(topic_id).expect("UTF-8 Text")
        })
        .collect();

    Ok(topic_ids)
}

#[cfg(test)]
mod tests {
    deserialize_test!(v0_pubsub_ls_0, PubsubLsResponse);
    deserialize_test!(v0_pubsub_ls_1, PubsubLsResponse);
    deserialize_test!(v0_pubsub_peers_0, PubsubPeersResponse);
    deserialize_test!(v0_pubsub_sub_0, PubsubSubResponse);
    deserialize_test!(v0_pubsub_sub_1, PubsubSubResponse);
}
