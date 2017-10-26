#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DnsResponse {
    pub path: String,
}
