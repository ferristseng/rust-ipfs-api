use response::serde;


pub type FilesCpResponse = ();


pub type FilesFlushResponse = ();


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FilesEntry {
    pub name: String,

    #[serde(rename = "Type")]
    pub typ: isize,
    pub size: i64,
    pub hash: String,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FilesLsResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub entries: Vec<FilesEntry>,
}


pub type FilesMkdirResponse = ();


pub type FilesMvResponse = ();


pub type FilesReadResponse = String;


pub type FilesRmResponse = ();


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FilesStatResponse {
    pub hash: String,
    pub size: u64,
    pub cumulative_size: u64,
    pub blocks: isize,

    #[serde(rename = "Type")]
    pub typ: String,
}


pub type FilesWriteResponse = ();


#[cfg(test)]
mod tests {
    deserialize_test!(v0_files_ls_0, FilesLsResponse);
    deserialize_test!(v0_files_stat_0, FilesStatResponse);
}
