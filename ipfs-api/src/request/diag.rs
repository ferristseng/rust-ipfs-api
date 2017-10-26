use request::ApiRequest;


pub struct DiagCmdsClear;

impl_skip_serialize!(DiagCmdsClear);

impl ApiRequest for DiagCmdsClear {
    #[inline]
    fn path() -> &'static str {
        "/diag/cmds/clear"
    }
}


#[derive(Serialize)]
pub struct DiagCmdsSetTime<'a> {
    #[serde(rename = "arg")]
    pub time: &'a str,
}

impl<'a> ApiRequest for DiagCmdsSetTime<'a> {
    #[inline]
    fn path() -> &'static str {
        "/diag/cmds/set-time"
    }
}


pub struct DiagSys;

impl_skip_serialize!(DiagSys);

impl ApiRequest for DiagSys {
    #[inline]
    fn path() -> &'static str {
        "/diag/sys"
    }
}
