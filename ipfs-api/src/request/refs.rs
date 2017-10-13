use request::ApiRequest;


pub struct RefsLocal;

impl_skip_serialize!(RefsLocal);

impl ApiRequest for RefsLocal {
    #[inline]
    fn path() -> &'static str {
        "/refs/local"
    }
}
