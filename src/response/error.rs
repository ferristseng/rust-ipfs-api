#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Error {
    pub message: String,
    pub code: u8
}
