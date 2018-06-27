// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

#![recursion_limit = "128"]

//! Rust library for connecting to the IPFS HTTP API using tokio.
//!
//! ## Usage
//!
//! ```toml
//! [dependencies]
//! ipfs-api = "0.5.0-alpha1"
//! ```
//!
//! ## Examples
//!
//! Write a file to IPFS:
//!
//! ```no_run
//! # extern crate hyper;
//! # extern crate ipfs_api;
//! #
//! use hyper::rt::Future;
//! use ipfs_api::IpfsClient;
//! use std::io::Cursor;
//!
//! # fn main() {
//! let client = IpfsClient::default();
//! let data = Cursor::new("Hello World!");
//!
//! let req = client
//!     .add(data)
//!     .map(|res| {
//!         println!("{}", res.hash);
//!     })
//!     .map_err(|e| eprintln!("{}", e));
//!
//! hyper::rt::run(req);
//! # }
//! ```
//!
//! Read a file from IPFS:
//!
//! ```no_run
//! # extern crate futures;
//! # extern crate hyper;
//! # extern crate ipfs_api;
//! #
//! use futures::{Future, Stream};
//! use ipfs_api::IpfsClient;
//! use std::io::{self, Write};
//!
//! # fn main() {
//! let client = IpfsClient::default();
//!
//! let req = client
//!     .get("/test/file.json")
//!     .concat2()
//!     .map(|res| {
//!         let out = io::stdout();
//!         let mut out = out.lock();
//!
//!         out.write_all(&res).unwrap();
//!     })
//!     .map_err(|e| eprintln!("{}", e));
//!
//! hyper::rt::run(req);
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
extern crate http;
extern crate hyper;
extern crate hyper_multipart_rfc7578 as hyper_multipart;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_urlencoded;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_io;

pub use client::IpfsClient;
pub use request::{KeyType, Logger, LoggingLevel, ObjectTemplate};

mod request;
pub mod response;
mod client;
mod header;
mod read;
