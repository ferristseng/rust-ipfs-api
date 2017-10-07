#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct KeyGenResponse {
    name: String,
    id: String
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct KeyListResponse {
    #[serde(default)]
    pub keys: Vec<KeyGenResponse>
}


#[cfg(test)]
mod tests {
    use super::{KeyGenResponse, KeyListResponse};


    deserialize_test!(v0_key_gen_0, KeyGenResponse);
    deserialize_test!(v0_key_list_0, KeyListResponse);
}
