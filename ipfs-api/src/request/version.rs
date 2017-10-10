use request::ApiRequest;


pub struct Version;

impl_skip_serialize!(Version);

impl ApiRequest for Version {
    #[inline]
    fn path() -> &'static str {
        "/version"
    }
}
