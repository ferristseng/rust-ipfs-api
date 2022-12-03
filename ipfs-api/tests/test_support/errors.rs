// Copyright 2022 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use passivized_docker_engine_client::errors::{DecCreateError, DecUseError};

#[derive(Debug, thiserror::Error)]
pub enum TestError {
    #[error("Docker client creation error: {0}")]
    DockerClientCreate(#[from] DecCreateError),

    #[error("Docker client error: {0}")]
    DockerClientUse(DecUseError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

// Doesn't conform to the trait requirements of #[from]
impl From<DecUseError> for TestError {
    fn from(other: DecUseError) -> Self {
        Self::DockerClientUse(other)
    }
}
