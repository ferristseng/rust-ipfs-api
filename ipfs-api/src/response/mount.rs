#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MountResponse {
    #[serde(rename = "IPFS")]
    pub ipfs: String,

    #[serde(rename = "IPNS")]
    pub ipns: String,

    pub fuse_allow_other: bool,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_mount_0, MountResponse);
}
