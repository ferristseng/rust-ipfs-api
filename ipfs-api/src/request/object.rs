use request::ApiRequest;


#[derive(Serialize)]
pub struct ObjectDiff<'a> {
    #[serde(rename = "arg")]
    pub key0: &'a str,

    #[serde(rename = "arg")]
    pub key1: &'a str,
}

impl<'a> ApiRequest for ObjectDiff<'a> {
    #[inline]
    fn path() -> &'static str {
        "/object/diff"
    }
}


#[derive(Serialize)]
pub struct ObjectGet<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,
}

impl<'a> ApiRequest for ObjectGet<'a> {
    #[inline]
    fn path() -> &'static str {
        "/object/get"
    }
}


#[derive(Serialize)]
pub struct ObjectLinks<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,
}

impl<'a> ApiRequest for ObjectLinks<'a> {
    #[inline]
    fn path() -> &'static str {
        "/object/links"
    }
}


#[derive(Serialize)]
pub struct ObjectStat<'a> {
    #[serde(rename = "arg")]
    pub key: &'a str,
}

impl<'a> ApiRequest for ObjectStat<'a> {
    #[inline]
    fn path() -> &'static str {
        "/object/stat"
    }
}


#[cfg(test)]
mod tests {
    use super::ObjectDiff;

    serialize_url_test!(
        test_serializes_0,
        ObjectDiff {
            key0: "test",
            key1: "test2",
        },
        "arg=test&arg=test2"
    );
}
