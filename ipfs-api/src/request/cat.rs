use request::ApiRequest;


#[derive(Serialize)]
pub struct Cat<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,
}

impl<'a> ApiRequest for Cat<'a> {
    #[inline]
    fn path() -> &'static str {
        "/cat"
    }
}
