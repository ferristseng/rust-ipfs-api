use crate::response::serde;
use crate::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SwarmConnectResponse {
    #[serde(deserialize_with = "serde::deserialize_vec")]
    pub strings: Vec<String>,
}

#[cfg(test)]
mod tests {
    deserialize_test!(v0_swarm_connect_0, SwarmConnectResponse);
}
