use request::ApiRequest;


#[derive(Serialize)]
pub struct ObjectDiff<'a> {
    #[serde(rename = "arg")]
    pub path0: &'a str,

    #[serde(rename = "arg")]
    pub path1: &'a str,
}

impl<'a> ApiRequest for ObjectDiff<'a> {
    #[inline]
    fn path() -> &'static str {
        "/object/diff"
    }
}


#[cfg(test)]
mod tests {
    use super::ObjectDiff;

    serialize_url_test!(
        test_serializes_0,
        ObjectDiff {
            path0: "test",
            path1: "test2",
        },
        "arg=test&arg=test2"
    );
}
