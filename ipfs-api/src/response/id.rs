use response::serde;


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IdResponse {
    #[serde(rename = "ID")]
    pub id: String,

    pub public_key: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub addresses: Vec<String>,

    pub agent_version: String,
    pub protocol_version: String,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_id_0, IdResponse);
}
