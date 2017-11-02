// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use request::ApiRequest;


pub struct FilestoreDups;

impl_skip_serialize!(FilestoreDups);

impl ApiRequest for FilestoreDups {
    #[inline]
    fn path() -> &'static str {
        "/filestore/dups"
    }
}


pub struct FilestoreLs;

impl_skip_serialize!(FilestoreLs);

impl ApiRequest for FilestoreLs {
    #[inline]
    fn path() -> &'static str {
        "/filestore/ls"
    }
}


pub struct FilestoreVerify;

impl_skip_serialize!(FilestoreVerify);

impl ApiRequest for FilestoreVerify {
    #[inline]
    fn path() -> &'static str {
        "/filestore/verify"
    }
}
