use request::ApiRequest;


#[derive(Serialize)]
pub struct BlockGet<'a> {
    #[serde(rename = "arg")]
    pub hash: &'a str,
}

impl<'a> ApiRequest for BlockGet<'a> {
    #[inline]
    fn path() -> &'static str {
        "/block/get"
    }
}


#[derive(Serialize)]
pub struct BlockRm<'a> {
    #[serde(rename = "arg")]
    pub hash: &'a str,
}

impl<'a> ApiRequest for BlockRm<'a> {
    #[inline]
    fn path() -> &'static str {
        "/block/rm"
    }
}


#[derive(Serialize)]
pub struct BlockStat<'a> {
    #[serde(rename = "arg")]
    pub hash: &'a str,
}

impl<'a> ApiRequest for BlockStat<'a> {
    #[inline]
    fn path() -> &'static str {
        "/block/stat"
    }
}
