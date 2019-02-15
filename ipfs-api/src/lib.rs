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
//! ipfs-api = "0.5.1"
//! ```
//!
//! You can use `actix-web` as a backend instead of `hyper`.
//!
//! ```toml
//! [dependencies]
//! ipfs-api = { version = "0.5.1", features = ["actix"], default-features = false }
//! ```
//!
//! ## Examples
//!
//! ### Writing a file to IPFS
//!
//! #### With Hyper
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
//! #### With Actix
//!
//! ```no_run
//! # extern crate actix_web;
//! # extern crate futures;
//! # extern crate ipfs_api;
//! #
//! use futures::future::Future;
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
//! actix_web::actix::run(|| {
//!     req.then(|_| {
//!         actix_web::actix::System::current().stop();
//!         Ok(())
//!     })
//! });
//! # }
//! ```
//!
//! ### Reading a file from IPFS
//!
//! #### With Hyper
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
//! #### With Actix
//!
//! ```no_run
//! # extern crate futures;
//! # extern crate actix_web;
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
//! actix_web::actix::run(|| {
//!     req.then(|_| {
//!         actix_web::actix::System::current().stop();
//!         Ok(())
//!     })
//! });
//! # }
//! ```
//!
//! ### Additional Examples
//!
//! There are also a bunch of examples included in the project, which
//! I used for testing
//!
//! For a list of examples, run:
//!
//! ```sh
//! $ cargo run --example
//! ```
//!
//! You can run any of the examples with cargo:
//!
//! ```sh
//! $ cargo run -p ipfs-api --example add_file
//! ```
//!
//! To run an example with the `actix-web` backend, use:
//!
//! ```sh
//! $ cargo run -p ipfs-api --features actix --no-default-features --example add_file
//! ```
//!

#[cfg(feature = "actix")]
extern crate actix_multipart_rfc7578 as actix_multipart;
#[cfg(feature = "actix")]
extern crate actix_web;

#[cfg(feature = "hyper")]
extern crate hyper;
#[cfg(feature = "hyper")]
extern crate hyper_multipart_rfc7578 as hyper_multipart;

extern crate bytes;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate http;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate dirs;
extern crate multiaddr;
extern crate serde_json;
extern crate serde_urlencoded;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_io;
extern crate walkdir;

pub use client::IpfsClient;
pub use request::{KeyType, Logger, LoggingLevel, ObjectTemplate};

mod client;
mod header;
mod read;
pub mod request;
pub mod response;
