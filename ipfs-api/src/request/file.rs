use request::ApiRequest;


#[derive(Serialize)]
pub struct FileLs<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,
}

impl<'a> ApiRequest for FileLs<'a> {
    #[inline]
    fn path() -> &'static str {
        "/file/ls"
    }
}
