// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::request::ApiRequest;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum DagCodec {
    #[serde(rename = "dag-json")]
    Json,
    #[serde(rename = "dag-cbor")]
    Cbor,
}

#[cfg_attr(feature = "with-builder", derive(TypedBuilder))]
#[derive(Serialize, Default)]
pub struct DagGet<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,

    /// Format that the object will be encoded as. Default: dag-json. Required: no.
    #[serde(rename = "output-codec")]
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub codec: Option<DagCodec>,
}

impl<'a> ApiRequest for DagGet<'a> {
    const PATH: &'static str = "/dag/get";
}

#[cfg_attr(feature = "with-builder", derive(TypedBuilder))]
#[derive(Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct DagPut<'a> {
    /// Codec that the stored object will be encoded with. Default: dag-cbor. Required: no.
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub store_codec: Option<DagCodec>,
    /// Codec that the input object is encoded in. Default: dag-json. Required: no.
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub input_codec: Option<DagCodec>,
    /// Pin this object when adding. Required: no.
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub pin: Option<bool>,
    /// Hash function to use. Default: sha2-256. Required: no.
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub hash: Option<&'a str>,
}

impl ApiRequest for DagPut<'_> {
    const PATH: &'static str = "/dag/put";
}

#[cfg(test)]
mod tests {
    use super::*;

    serialize_url_test!(
        test_serializes_dag_get,
        DagGet {
            path: "bafkreiglbo2l5lp25vteuexq3svg5hoad76mehz4tlrbwheslvluxcd63a",
            ..Default::default()
        },
        "arg=bafkreiglbo2l5lp25vteuexq3svg5hoad76mehz4tlrbwheslvluxcd63a"
    );

    serialize_url_test!(
        test_serializes_dag_get_with_options,
        DagGet {
            path: "bafkreiglbo2l5lp25vteuexq3svg5hoad76mehz4tlrbwheslvluxcd63a",
            codec: Some(DagCodec::Cbor),
        },
        "arg=bafkreiglbo2l5lp25vteuexq3svg5hoad76mehz4tlrbwheslvluxcd63a&output-codec=dag-cbor"
    );

    serialize_url_test!(
        test_serializes_dag_put,
        DagPut {
            ..Default::default()
        },
        ""
    );

    serialize_url_test!(
        test_serializes_dag_put_with_options,
        DagPut {
            store_codec: Some(DagCodec::Json),
            input_codec: Some(DagCodec::Cbor),
            pin: Some(false),
            hash: Some("sha3_384"),
        },
        "store-codec=dag-json&input-codec=dag-cbor&pin=false&hash=sha3_384"
    );
}
