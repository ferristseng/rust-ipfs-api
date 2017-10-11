use response::serde;


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IpfsFile {
    pub hash: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub links: Vec<IpfsFileHeader>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IpfsFileHeader {
    pub name: String,
    pub hash: String,
    pub size: u64,

    #[serde(rename = "Type")]
    pub typ: u32,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LsResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub objects: Vec<IpfsFile>,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_ls_0, LsResponse);
    deserialize_test!(v0_ls_1, LsResponse);
}
