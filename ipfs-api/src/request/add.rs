use request::ApiRequest;


pub struct Add;

impl ApiRequest for Add {
    #[inline]
    fn path() -> &'static str {
        "/add"
    }
}
