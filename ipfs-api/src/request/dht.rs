use request::ApiRequest;


#[derive(Serialize)]
pub struct DhtFindPeer<'a> {
    #[serde(rename = "arg")]
    pub peer: &'a str,
}

impl<'a> ApiRequest for DhtFindPeer<'a> {
    #[inline]
    fn path() -> &'static str {
        "/dht/findpeer"
    }
}


#[derive(Serialize)]
pub struct DhtFindProvs<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,
}

impl<'a> ApiRequest for DhtFindProvs<'a> {
    #[inline]
    fn path() -> &'static str {
        "/dht/findprovs"
    }
}


#[derive(Serialize)]
pub struct DhtGet<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,
}

impl<'a> ApiRequest for DhtGet<'a> {
    #[inline]
    fn path() -> &'static str {
        "/dht/get"
    }
}


#[derive(Serialize)]
pub struct DhtProvide<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,
}

impl<'a> ApiRequest for DhtProvide<'a> {
    #[inline]
    fn path() -> &'static str {
        "/dht/provide"
    }
}


#[derive(Serialize)]
pub struct DhtPut<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,

    #[serde(rename = "arg")]
    pub value: &'a str,
}

impl<'a> ApiRequest for DhtPut<'a> {
    #[inline]
    fn path() -> &'static str {
        "/dht/put"
    }
}


#[derive(Serialize)]
pub struct DhtQuery<'a> {
    #[serde(rename = "arg")]
    pub peer: &'a str,
}

impl<'a> ApiRequest for DhtQuery<'a> {
    #[inline]
    fn path() -> &'static str {
        "/dht/query"
    }
}
