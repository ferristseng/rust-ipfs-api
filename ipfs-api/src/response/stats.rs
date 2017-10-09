use response::{BitswapStatResponse, RepoStatResponse};


pub type StatsBitswapResponse = BitswapStatResponse;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StatsBwResponse {
    pub total_in: u64,
    pub total_out: u64,
    pub rate_in: f64,
    pub rate_out: f64,
}


pub type StatsRepoResponse = RepoStatResponse;


#[cfg(test)]
mod tests {
    deserialize_test!(v0_stats_bw_0, StatsBwResponse);
}
