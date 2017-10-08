use response::{serde, IpfsHeader};
use std::collections::HashMap;


pub type ObjectDataResponse = Vec<u8>;


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectDiff {
    #[serde(rename = "Type")]
    pub typ: isize,

    pub path: String,

    #[serde(deserialize_with = "serde::deserialize_hashmap")]
    pub before: HashMap<String, String>,

    #[serde(deserialize_with = "serde::deserialize_hashmap")]
    pub after: HashMap<String, String>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectDiffResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub changes: Vec<ObjectDiff>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectLinksResponse {
    pub hash: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub links: Vec<IpfsHeader>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectNewResponse {
    pub hash: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub links: Vec<IpfsHeader>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectPatchAddLinkResponse {
    pub hash: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub links: Vec<IpfsHeader>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectPatchAppendDataResponse {
    pub hash: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub links: Vec<IpfsHeader>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectPatchRmLinkResponse {
    pub hash: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub links: Vec<IpfsHeader>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectPatchSetDataResponse {
    pub hash: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub links: Vec<IpfsHeader>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectPutResponse {
    pub hash: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub links: Vec<IpfsHeader>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectStatResponse {
    pub hash: String,
    pub num_links: isize,
    pub block_size: isize,
    pub links_size: isize,
    pub data_size: isize,
    pub cumulative_size: isize,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_object_diff_0, ObjectDiffResponse);
    deserialize_test!(v0_object_links_0, ObjectLinksResponse);
    deserialize_test!(v0_object_stat_0, ObjectStatResponse);
}
