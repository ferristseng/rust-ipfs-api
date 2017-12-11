// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use request::ApiRequest;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DiagCmdsClear;

impl_skip_serialize!(DiagCmdsClear);

impl ApiRequest for DiagCmdsClear {
    const path: &'static str = "/diag/cmds/clear";
}


#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DiagCmdsSetTime<'a> {
    #[serde(rename = "arg")]
    pub time: &'a str,
}

impl<'a> ApiRequest for DiagCmdsSetTime<'a> {
    const path: &'static str = "/diag/cmds/set-time";
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DiagSys;

impl_skip_serialize!(DiagSys);

impl ApiRequest for DiagSys {
    const path: &'static str = "/diag/sys";
}
