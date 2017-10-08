use response::{serde, IpfsHeader};
use std::collections::HashMap;


pub type ObjectDataResponse = Vec<u8>;


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectDiff {
    #[serde(rename = "Type")]
    typ: isize,

    path: String,

    #[serde(deserialize_with = "serde::deserialize_hashmap")]
    before: HashMap<String, String>,

    #[serde(deserialize_with = "serde::deserialize_hashmap")]
    after: HashMap<String, String>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectDiffResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    changes: Vec<ObjectDiff>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectLinksResponse {
    hash: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    links: Vec<IpfsHeader>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectNewResponse {
    hash: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    links: Vec<IpfsHeader>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectPatchAddLinkResponse {
    hash: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    links: Vec<IpfsHeader>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectPatchAppendDataResponse {
    hash: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    links: Vec<IpfsHeader>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectPatchRmLinkResponse {
    hash: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    links: Vec<IpfsHeader>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectPatchSetDataResponse {
    hash: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    links: Vec<IpfsHeader>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectPutResponse {
    hash: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    links: Vec<IpfsHeader>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectStatResponse {
    hash: String,
    num_links: isize,
    block_size: isize,
    links_size: isize,
    data_size: isize,
    cumulative_size: isize,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_object_diff_0, ObjectDiffResponse);
    deserialize_test!(v0_object_links_0, ObjectLinksResponse);
    deserialize_test!(v0_object_stat_0, ObjectStatResponse);
}
