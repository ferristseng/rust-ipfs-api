use response::serde;


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DhtPeerResponse {
    #[serde(rename = "ID")]
    pub id: String,

    pub addrs: String,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DhtFindPeerResponse {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "Type")]
    pub typ: isize,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub responses: Vec<DhtPeerResponse>,

    pub extra: String,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DhtFindProvsResponse {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "Type")]
    pub typ: isize,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub responses: Vec<DhtPeerResponse>,

    pub extra: String,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DhtGetResponse {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "Type")]
    pub typ: isize,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub responses: Vec<DhtPeerResponse>,

    pub extra: String,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DhtProvideResponse {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "Type")]
    pub typ: isize,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub responses: Vec<DhtPeerResponse>,

    pub extra: String,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DhtPutResponse {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "Type")]
    pub typ: isize,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub responses: Vec<DhtPeerResponse>,

    pub extra: String,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DhtQueryResponse {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "Type")]
    pub typ: isize,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub responses: Vec<DhtPeerResponse>,

    pub extra: String,
}
