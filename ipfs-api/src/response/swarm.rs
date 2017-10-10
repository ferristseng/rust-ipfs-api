use response::serde;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SwarmAddrsLocalResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub strings: Vec<String>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SwarmAddrsConnectResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub strings: Vec<String>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SwarmAddrsDisconnectResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub strings: Vec<String>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SwarmFiltersAddResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub strings: Vec<String>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SwarmFiltersRmResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub strings: Vec<String>,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SwarmPeerStream {
    pub protocol: String,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SwarmPeer {
    pub addr: String,
    pub peer: String,
    pub latency: String,
    pub muxer: String,

    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub streams: Vec<SwarmPeerStream>,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SwarmPeersResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub peers: Vec<SwarmPeer>,
}


#[cfg(test)]
mod tests {
    deserialize_test!(v0_swarm_addrs_local_0, SwarmAddrsLocalResponse);
    deserialize_test!(v0_swarm_peers_0, SwarmPeersResponse);
    deserialize_test!(v0_swarm_peers_1, SwarmPeersResponse);
    deserialize_test!(v0_swarm_peers_2, SwarmPeersResponse);
}
