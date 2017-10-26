// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

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
