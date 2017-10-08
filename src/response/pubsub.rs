use response::serde;


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PubsubLsResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub strings: Vec<String>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PubsubPeersResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub strings: Vec<String>,
}


pub type PubsubPubResponse = ();


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PubsubMessage {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub from: Vec<u8>,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub data: Vec<u8>,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub seqno: Vec<u8>,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub topic_ids: Vec<String>,

    #[serde(rename = "XXX_unrecognized")]
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub unrecognized: Vec<u8>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PubsubSubResponse {
    pub message: Option<PubsubMessage>,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_pubsub_ls_0, PubsubLsResponse);
}
