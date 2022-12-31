// Copyright 2022 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::test_support::errors::TestError;
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// From a path foo/bar/qux.txt, remove foo/
fn strip_path_prefix(path: &Path) -> PathBuf {
    let base = path
        .components()
        .into_iter()
        .next()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap();

    path.strip_prefix(base).unwrap().to_owned()
}

async fn read_default_conf_template() -> Result<String, TestError> {
    let raw_file = PathBuf::from_str(file!()).unwrap();

    let file = if raw_file.exists() {
        raw_file
    } else {
        // Obviously this file exists, but the working dir might be confused by the tools running the test.
        strip_path_prefix(&raw_file)
    };

    let absolute_file = file.canonicalize().unwrap();

    let parent = absolute_file.parent().unwrap().to_owned();

    let absolute = parent.canonicalize().unwrap().to_owned();

    let path = absolute.join("default-template.conf");

    Ok(tokio::fs::read_to_string(path).await?)
}

pub fn set_config_permissions(path: &Path) -> Result<(), std::io::Error> {
    use std::os::unix::fs::PermissionsExt;

    let f = std::fs::File::open(path)?;
    f.set_permissions(PermissionsExt::from_mode(0o644))
}

pub async fn write_default_conf(ip: &str, output: &Path) -> Result<(), TestError> {
    let template = read_default_conf_template().await?;
    let config = template.replace("replaced_at_runtime", ip);

    println!("Writing {}", output.to_str().unwrap());

    tokio::fs::write(output, config).await?;

    Ok(())
}
