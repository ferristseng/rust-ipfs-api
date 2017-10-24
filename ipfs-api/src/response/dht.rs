use response::serde;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DhtPeerResponse {
    #[serde(rename = "ID")]
    pub id: String,

    pub addrs: String,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DhtMessage {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "Type")]
    pub typ: isize,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub responses: Vec<DhtPeerResponse>,

    pub extra: String,
}

pub type DhtFindPeerResponse = DhtMessage;

pub type DhtFindProvsResponse = DhtMessage;

pub type DhtGetResponse = DhtMessage;

pub type DhtPutResponse = DhtMessage;

pub type DhtQueryResponse = DhtMessage;
