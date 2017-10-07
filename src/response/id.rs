#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IdResponse {
    #[serde(rename = "ID")]
    id: String,

    public_key: String,

    #[serde(default)]
    addresses: Vec<String>,

    agent_version: String,
    protocol_version: String
}


#[cfg(test)]
mod tests {
    use super::IdResponse;


    deserialize_test!(v0_id_0, IdResponse);
}
