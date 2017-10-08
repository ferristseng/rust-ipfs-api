use response::serde;


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BitswapLedgerResponse {
    pub peer: String,
    pub value: f64,
    pub sent: u64,
    pub recv: u64,
    pub exchange: u64,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BitswapStatResponse {
    pub provide_buf_len: isize,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub wantlist: Vec<String>,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub peers: Vec<String>,

    pub blocks_received: isize,
    pub data_received: u64,
    pub blocks_sent: isize,
    pub data_sent: u64,
    pub dup_blks_received: isize,
    pub dup_data_received: u64,
}


pub type BitswapUnwantResponse = String;


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BitswapWantlistResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub keys: Vec<String>,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_bitswap_stat_0, BitswapStatResponse);
}
