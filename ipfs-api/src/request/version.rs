use request::ApiRequest;


pub struct Version;

impl ApiRequest for Version {
    #[inline]
    fn path() -> &'static str {
        "/version"
    }
}
