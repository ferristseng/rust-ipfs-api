use request::ApiRequest;


#[derive(Serialize)]
pub struct DagGet<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,
}

impl<'a> ApiRequest for DagGet<'a> {
    #[inline]
    fn path() -> &'static str {
        "/dag/get"
    }
}
