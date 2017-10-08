use request::ApiRequest;


pub struct LsRequest<'a>(pub Option<&'a str>);

impl<'a> ApiRequest for LsRequest<'a> {
    #[inline]
    fn path() -> &'static str {
        "/ls"
    }
}
