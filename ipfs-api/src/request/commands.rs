use request::ApiRequest;


pub struct Commands;

impl ApiRequest for Commands {
    #[inline]
    fn path() -> &'static str {
        "/commands"
    }
}
