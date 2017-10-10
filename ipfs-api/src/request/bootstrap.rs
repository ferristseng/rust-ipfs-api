use request::ApiRequest;


pub struct BootstrapAddDefault;

impl_skip_serialize!(BootstrapAddDefault);

impl ApiRequest for BootstrapAddDefault {
    #[inline]
    fn path() -> &'static str {
        "/bootstrap/add/default"
    }
}


pub struct BootstrapList;

impl_skip_serialize!(BootstrapList);

impl ApiRequest for BootstrapList {
    #[inline]
    fn path() -> &'static str {
        "/bootstrap/list"
    }
}


pub struct BootstrapRmAll;

impl_skip_serialize!(BootstrapRmAll);

impl ApiRequest for BootstrapRmAll {
    #[inline]
    fn path() -> &'static str {
        "/bootstrap/rm/all"
    }
}
