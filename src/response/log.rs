#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LogLevelResponse {
    pub message: String
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LogLsResponse {
    #[serde(default)]
    pub strings: Vec<String>
}


#[cfg(test)]
mod tests {
    use super::LogLsResponse;


    deserialize_test!(v0_log_ls_0, LogLsResponse);
}
