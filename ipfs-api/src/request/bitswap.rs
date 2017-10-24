use request::ApiRequest;


#[derive(Serialize)]
pub struct BitswapLedger<'a> {
    #[serde(rename = "arg")]
    pub peer: &'a str,
}

impl<'a> ApiRequest for BitswapLedger<'a> {
    #[inline]
    fn path() -> &'static str {
        "/bitswap/ledger"
    }
}


pub struct BitswapStat;

impl_skip_serialize!(BitswapStat);

impl ApiRequest for BitswapStat {
    #[inline]
    fn path() -> &'static str {
        "/bitswap/stat"
    }
}


#[derive(Serialize)]
pub struct BitswapUnwant<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,
}

impl<'a> ApiRequest for BitswapUnwant<'a> {
    #[inline]
    fn path() -> &'static str {
        "/bitswap/stat"
    }
}


#[derive(Serialize)]
pub struct BitswapWantlist<'a> {
    pub peer: Option<&'a str>,
}

impl<'a> ApiRequest for BitswapWantlist<'a> {
    #[inline]
    fn path() -> &'static str {
        "/bitswap/wantlist"
    }
}
