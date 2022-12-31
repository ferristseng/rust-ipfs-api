// Copyright 2022 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

pub mod nginx {
    pub const IMAGE: &str = "nginx";
    pub const TAG: &str = "1.23-alpine";
}

/// Extract version triple from an IPFS/Kubo Docker image tag.
///
/// Result should match VersionResponse::version
pub fn extract_version(image_tag: &str) -> String {
    image_tag.strip_prefix('v').unwrap().to_string()
}
