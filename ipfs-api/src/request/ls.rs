use request::ApiRequest;


#[derive(Serialize)]
pub struct Ls<'a> {
    #[serde(rename = "arg")]
    pub path: Option<&'a str>,
}

impl<'a> ApiRequest for Ls<'a> {
    #[inline]
    fn path() -> &'static str {
        "/ls"
    }
}


#[cfg(test)]
mod tests {
    use super::Ls;

    serialize_url_test!(test_serializes_0, Ls { path: Some("test") }, "arg=test");
    serialize_url_test!(test_serializes_1, Ls { path: None }, "");
}
