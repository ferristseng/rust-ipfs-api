use request::ApiRequest;


pub struct Add;

impl_skip_serialize!(Add);

impl ApiRequest for Add {
    #[inline]
    fn path() -> &'static str {
        "/add"
    }
}
