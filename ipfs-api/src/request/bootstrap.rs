use request::ApiRequest;


pub struct BootstrapAddDefault;

impl ApiRequest for BootstrapAddDefault {
    #[inline]
    fn path() -> &'static str {
        "/bootstrap/add/default"
    }
}


pub struct BootstrapList;

impl ApiRequest for BootstrapList {
    #[inline]
    fn path() -> &'static str {
        "/bootstrap/list"
    }
}


pub struct BootstrapRmAll;

impl ApiRequest for BootstrapRmAll {
    #[inline]
    fn path() -> &'static str {
        "/bootstrap/rm/all"
    }
}
