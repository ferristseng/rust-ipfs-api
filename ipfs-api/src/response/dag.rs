use response::serde;
use std::collections::HashMap;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DagIpfsHeader {
    pub name: String,
    pub size: u64,

    #[serde(deserialize_with = "serde::deserialize_hashmap")]
    pub cid: HashMap<String, String>,
}


#[derive(Debug, Deserialize)]
pub struct DagGetResponse {
    pub data: Option<String>,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub links: Vec<DagIpfsHeader>,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DagPutResponse {
    pub cid: String,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_dag_get_0, DagGetResponse);
}
