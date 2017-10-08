use response::{serde, IpfsFile};


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
