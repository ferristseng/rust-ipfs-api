use request::ApiRequest;


pub struct Ls<'a>(pub Option<&'a str>);

impl<'a> ApiRequest for Ls<'a> {
    #[inline]
    fn path() -> &'static str {
        "/ls"
    }
}
