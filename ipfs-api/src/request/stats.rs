// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::request::ApiRequest;

pub struct StatsBitswap;

impl_skip_serialize!(StatsBitswap);

impl ApiRequest for StatsBitswap {
    const PATH: &'static str = "/stats/bitswap";
}

pub struct StatsBw;

impl_skip_serialize!(StatsBw);

impl ApiRequest for StatsBw {
    const PATH: &'static str = "/stats/bw";
}

pub struct StatsRepo;

impl_skip_serialize!(StatsRepo);

impl ApiRequest for StatsRepo {
    const PATH: &'static str = "/stats/repo";
}
