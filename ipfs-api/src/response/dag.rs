pub type DagGetResponse = String;


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DagPutResponse {
    pub cid: String,
}
