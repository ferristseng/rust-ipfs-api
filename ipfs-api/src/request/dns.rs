use request::ApiRequest;


#[derive(Serialize)]
pub struct Dns<'a> {
    #[serde(rename = "arg")]
    pub link: &'a str,

    pub recursive: bool,
}

impl<'a> ApiRequest for Dns<'a> {
    #[inline]
    fn path() -> &'static str {
        "/dns"
    }
}
