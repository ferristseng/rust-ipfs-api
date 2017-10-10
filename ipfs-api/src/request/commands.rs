use request::ApiRequest;


pub struct Commands;

impl_skip_serialize!(Commands);

impl ApiRequest for Commands {
    #[inline]
    fn path() -> &'static str {
        "/commands"
    }
}
