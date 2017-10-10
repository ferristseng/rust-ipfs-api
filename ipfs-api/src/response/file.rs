use response::{serde, IpfsHeader};
use std::collections::HashMap;


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IpfsDetailedFile {
    pub hash: String,
    pub size: u64,

    #[serde(rename = "Type")]
    pub typ: String,

    #[serde(default)]
    pub links: Vec<IpfsHeader>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FileLsResponse {
    #[serde(deserialize_with = "serde::deserialize_hashmap")]
    pub arguments: HashMap<String, String>,

    #[serde(deserialize_with = "serde::deserialize_hashmap")]
    pub objects: HashMap<String, IpfsDetailedFile>,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_file_ls_0, FileLsResponse);
    deserialize_test!(v0_file_ls_1, FileLsResponse);
}
