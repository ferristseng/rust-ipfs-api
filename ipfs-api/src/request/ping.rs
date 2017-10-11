use request::ApiRequest;


#[derive(Serialize)]
pub struct Ping<'a> {
    #[serde(rename = "arg")]
    pub peer: &'a str,

    pub count: Option<usize>,
}

impl<'a> ApiRequest for Ping<'a> {
    #[inline]
    fn path() -> &'static str {
        "/ping"
    }
}
