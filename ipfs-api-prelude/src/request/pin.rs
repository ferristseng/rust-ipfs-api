// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::request::ApiRequest;
use serde::Serialize;

#[derive(Serialize)]
pub struct PinAdd<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,

    pub recursive: Option<bool>,
    pub progress: bool,
}

impl<'a> ApiRequest for PinAdd<'a> {
    const PATH: &'static str = "/pin/add";
}

#[derive(Serialize)]
pub struct PinLs<'a> {
    #[serde(rename = "arg")]
    pub key: Option<&'a str>,

    #[serde(rename = "type")]
    pub typ: Option<&'a str>,
}

impl<'a> ApiRequest for PinLs<'a> {
    const PATH: &'static str = "/pin/ls";
}

#[derive(Serialize)]
pub struct PinRm<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,

    pub recursive: bool,
}

impl<'a> ApiRequest for PinRm<'a> {
    const PATH: &'static str = "/pin/rm";
}

#[derive(Serialize)]
pub struct PinRemoteAdd<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,

    pub service: Option<&'a str>,
    pub name: Option<&'a str>,
    pub background: Option<bool>,
}

impl<'a> ApiRequest for PinRemoteAdd<'a> {
    const PATH: &'static str = "/pin/remote/add";
}

#[derive(Serialize)]
pub struct PinRemoteLs<'a> {
    pub service: Option<&'a str>,
    pub name: Option<&'a str>,
    pub cid: Option<Cids<'a>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_pin_status"
    )]
    pub status: Option<&'a [PinStatus]>,
}

impl<'a> ApiRequest for PinRemoteLs<'a> {
    const PATH: &'static str = "/pin/remote/ls";
}

#[derive(Serialize)]
pub struct PinRemoteRm<'a> {
    pub service: Option<&'a str>,
    pub name: Option<&'a str>,
    pub cid: Option<Cids<'a>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_pin_status"
    )]
    pub status: Option<&'a [PinStatus]>,
    pub force: Option<bool>,
}

impl<'a> ApiRequest for PinRemoteRm<'a> {
    const PATH: &'static str = "/pin/remote/rm";
}

pub struct Cids<'a>(&'a [&'a str]);

impl<'a> From<&'a [&'a str]> for Cids<'a> {
    fn from(data: &'a [&'a str]) -> Self {
        Self(data)
    }
}

impl<'a> Serialize for Cids<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let cids = self.0.join(",");
        serializer.serialize_str(&cids)
    }
}

#[derive(Serialize)]
pub enum PinStatus {
    Queued,
    Pinning,
    Pinned,
    Failed,
}

impl Default for PinStatus {
    fn default() -> Self {
        Self::Pinned
    }
}

fn serialize_pin_status<S>(
    pin_status: &Option<&[PinStatus]>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let pin_status = pin_status.unwrap();
    let pin_status = pin_status
        .iter()
        .map(|item| match *item {
            PinStatus::Failed => "failed",
            PinStatus::Queued => "queued",
            PinStatus::Pinning => "pinning",
            PinStatus::Pinned => "pinned",
        })
        .collect::<Vec<&str>>()
        .join(",");
    serializer.serialize_str(&format!("[{pin_status}]"))
}

#[cfg(test)]
mod tests {
    use super::*;

    serialize_url_test!(
        test_serializes_pin_remote_rm,
        PinRemoteRm {
            service: Some("Pinata"),
            name: None,
            cid: Some((&vec!["bafybeiaq3hspbuvhvg7nlxjjvsnzit6m6hevrjwedoj4jbx6uycgkkexni"] as &[&str]).into()),
            status: Some(&vec![PinStatus::Pinned, PinStatus::Pinning]),
            force: Some(true)
        },
        "service=Pinata&cid=bafybeiaq3hspbuvhvg7nlxjjvsnzit6m6hevrjwedoj4jbx6uycgkkexni&status=%5Bpinned%2Cpinning%5D&force=true"
    );

    serialize_url_test!(
        test_serializes_pin_remote_rm_multi_cid,
        PinRemoteRm {
            service: Some("Pinata"),
            name: None,
            cid: Some((&vec!["bafybeiaq3hspbuvhvg7nlxjjvsnzit6m6hevrjwedoj4jbx6uycgkkexni", "QmfWC6JwVxmjVQfPpSiTsxFaSBdPTtFCd1B4aqMqRgaeMU"] as &[&str]).into()),
            status:None,
            force: None
        },
        "service=Pinata&cid=bafybeiaq3hspbuvhvg7nlxjjvsnzit6m6hevrjwedoj4jbx6uycgkkexni%2CQmfWC6JwVxmjVQfPpSiTsxFaSBdPTtFCd1B4aqMqRgaeMU"
    );
}
