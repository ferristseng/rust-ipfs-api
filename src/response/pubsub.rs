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
    pub from: String,
    pub data: String,
    pub seqno: String,
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
    deserialize_test!(v0_pubsub_ls_1, PubsubLsResponse);
    deserialize_test!(v0_pubsub_peers_0, PubsubPeersResponse);
    deserialize_test!(v0_pubsub_sub_0, PubsubSubResponse);
}
