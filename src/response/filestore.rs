#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FilestoreDupsResponse {
    #[serde(rename = "Ref")]
    pub refr: String,

    pub err: String
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FilestoreLsResponse {
    pub status: i32,
    pub error_msg: String,
    pub key: String,
    pub file_path: String,
    pub offset: u64,
    pub size: u64
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FilestoreVerifyResponse {
    pub status: i32,
    pub error_msg: String,
    pub key: String,
    pub file_path: String,
    pub offset: u64,
    pub size: u64
}
