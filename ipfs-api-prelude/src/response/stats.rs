// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::response::{BitswapStatResponse, RepoStatResponse};
use crate::serde::Deserialize;

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
