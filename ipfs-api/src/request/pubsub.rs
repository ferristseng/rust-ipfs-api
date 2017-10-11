use request::ApiRequest;


pub struct PubsubLs;

impl_skip_serialize!(PubsubLs);

impl ApiRequest for PubsubLs {
    #[inline]
    fn path() -> &'static str {
        "/pubsub/ls"
    }
}


#[derive(Serialize)]
pub struct PubsubPeers<'a> {
    #[serde(rename = "arg")]
    pub topic: Option<&'a str>
}

impl<'a> ApiRequest for PubsubPeers<'a> {
    #[inline]
    fn path() -> &'static str {
        "/pubsub/peers"
    }
}
