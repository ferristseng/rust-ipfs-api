use crate::request::ApiRequest;
use serde::Serialize;

#[derive(Serialize)]
pub struct SwarmConnect<'a> {
    #[serde(rename = "arg")]
    pub peer: &'a str,
}
impl<'a> ApiRequest for SwarmConnect<'a> {
    const PATH: &'static str = "/swarm/connect";
}
