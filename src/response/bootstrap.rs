#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BootstrapAddDefaultResponse {
    #[serde(default)]
    pub peers: Vec<String>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BootstrapListResponse {
    #[serde(default)]
    pub peers: Vec<String>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BootstrapRmAllResponse {
    #[serde(default)]
    pub peers: Vec<String>,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_bootstrap_list_0, BootstrapListResponse);
}
