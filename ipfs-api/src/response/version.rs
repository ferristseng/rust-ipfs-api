#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VersionResponse {
    pub version: String,
    pub commit: String,
    pub repo: String,
    pub system: String,
    pub golang: String,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_version_0, VersionResponse);
}
