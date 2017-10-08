use response::serde;


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LogLevelResponse {
    pub message: String,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LogLsResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub strings: Vec<String>,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_log_ls_0, LogLsResponse);
}
