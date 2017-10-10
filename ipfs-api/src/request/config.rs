use request::ApiRequest;


pub struct ConfigShow;

impl_skip_serialize!(ConfigShow);

impl ApiRequest for ConfigShow {
    #[inline]
    fn path() -> &'static str {
        "/config/show"
    }
}


pub struct ConfigEdit;

impl_skip_serialize!(ConfigEdit);

impl ApiRequest for ConfigEdit {
    #[inline]
    fn path() -> &'static str {
        "/config/edit"
    }
}
