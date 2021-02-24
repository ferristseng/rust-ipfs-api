// Copyright 2021 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

#[cfg(feature = "with-actix")]
pub use actix_rt::main;

#[cfg(feature = "with-actix")]
pub use ipfs_api_backend_actix as ipfs_api;

#[cfg(feature = "with-actix")]
pub use tokio_actix as tokio; // Compatibilty for actix-rt 1.0

#[cfg(feature = "with-hyper")]
pub use tokio::main;

#[cfg(feature = "with-hyper")]
pub use ipfs_api_backend_hyper as ipfs_api;

#[cfg(feature = "with-hyper")]
pub use tokio_hyper as tokio;
