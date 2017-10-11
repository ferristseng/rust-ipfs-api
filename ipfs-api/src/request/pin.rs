use request::ApiRequest;


#[derive(Serialize)]
pub struct PinLs<'a> {
    #[serde(rename = "arg")]
    pub key: Option<&'a str>,

    #[serde(rename = "type")]
    pub typ: Option<&'a str>,
}

impl<'a> ApiRequest for PinLs<'a> {
    #[inline]
    fn path() -> &'static str {
        "/pin/ls"
    }
}
