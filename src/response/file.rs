use response::IpfsFile;
use std::collections::HashMap;


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FileLsResponse {
    #[serde(default)]
    pub arguments: HashMap<String, String>,

    #[serde(default)]
    pub objects: HashMap<String, IpfsFile>,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_file_ls_0, FileLsResponse);
    deserialize_test!(v0_file_ls_1, FileLsResponse);
}
