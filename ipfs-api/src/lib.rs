// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

//! Rust library for connecting to the IPFS HTTP API using tokio.
//!
//! ## Usage
//!
//! ```toml
//! [dependencies]
//! ipfs-api = "0.4.0-alpha.1"
//! ```
//!
//! ## Examples
//!
//! Write a file to IPFS:
//!
//! ```no_run
//! # extern crate ipfs_api;
//! # extern crate tokio_core;
//! #
//! use ipfs_api::IpfsClient;
//! use std::io::Cursor;
//! use tokio_core::reactor::Core;
//!
//! # fn main() {
//! let mut core = Core::new().unwrap();
//! let client = IpfsClient::default(&core.handle());
//! let data = Cursor::new("Hello World!");
//!
//! let req = client.add(data);
//! let res = core.run(req).unwrap();
//!
//! println!("{}", res.hash);
//! # }
//! ```
//!
//! Read a file from IPFS:
//!
//! ```no_run
//! # extern crate futures;
//! # extern crate ipfs_api;
//! # extern crate tokio_core;
//! #
//! use futures::stream::Stream;
//! use ipfs_api::IpfsClient;
//! use std::io::{self, Write};
//! use tokio_core::reactor::Core;
//!
//! # fn main() {
//! let mut core = Core::new().unwrap();
//! let client = IpfsClient::default(&core.handle());
//!
//! let req = client.get("/test/file.json").concat2();
//! let res = core.run(req).unwrap();
//! let out = io::stdout();
//! let mut out = out.lock();
//!
//! out.write_all(&res).unwrap();
//! # }
//! ```
//!
//! There are also a bunch of examples included in the project, which
//! I used for testing
//!
//! You can run any of the examples with cargo:
//!
//! ```sh
//! $ cargo run -p ipfs-api --example add_file
//! ```

extern crate bytes;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate hyper;
extern crate hyper_multipart_rfc7578 as hyper_multipart;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_urlencoded;
extern crate tokio_core;
extern crate tokio_io;

pub use client::IpfsClient;
pub use request::{KeyType, Logger, LoggingLevel};

mod request;
pub mod response;
mod client;
mod header;
mod read;
