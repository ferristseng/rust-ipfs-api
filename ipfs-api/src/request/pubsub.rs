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
    pub topic: Option<&'a str>,
}

impl<'a> ApiRequest for PubsubPeers<'a> {
    #[inline]
    fn path() -> &'static str {
        "/pubsub/peers"
    }
}


#[derive(Serialize)]
pub struct PubsubPub<'a> {
    #[serde(rename = "arg")]
    pub topic: &'a str,

    #[serde(rename = "arg")]
    pub payload: &'a str,
}

impl<'a> ApiRequest for PubsubPub<'a> {
    #[inline]
    fn path() -> &'static str {
        "/pubsub/pub"
    }
}


#[derive(Serialize)]
pub struct PubsubSub<'a> {
    #[serde(rename = "arg")]
    pub topic: &'a str,

    pub discover: Option<bool>,
}

impl<'a> ApiRequest for PubsubSub<'a> {
    #[inline]
    fn path() -> &'static str {
        "/pubsub/sub"
    }
}
