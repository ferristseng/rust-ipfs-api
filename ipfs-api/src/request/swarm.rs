// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use request::ApiRequest;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SwarmAddrsLocal;

impl_skip_serialize!(SwarmAddrsLocal);

impl ApiRequest for SwarmAddrsLocal {
    const path: &'static str = "/swarm/addrs/local";
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SwarmPeers;

impl_skip_serialize!(SwarmPeers);

impl ApiRequest for SwarmPeers {
        const path: &'static str = "/swarm/peers";
}
