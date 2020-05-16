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
//! ipfs-api = "0.7.2"
//! ```
//!
//! You can use `actix-web` as a backend instead of `hyper`.
//!
//! ```toml
//! [dependencies]
//! ipfs-api = { version = "0.7.2", features = ["actix"], default-features = false }
//! ```
//!
//! ## Examples
//!
//! ### Writing a file to IPFS
//!
//! #### With Hyper
//!
//! ```no_run
//! use ipfs_api::IpfsClient;
//! use std::io::Cursor;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = IpfsClient::default();
//!     let data = Cursor::new("Hello World!");
//!
//!     match client.add(data).await {
//!         Ok(res) => println!("{}", res.hash),
//!         Err(e) => eprintln!("error adding file: {}", e)
//!     }
//! }
//! ```
//!
//! #### With Actix
//!
//! ```no_run
//! use ipfs_api::IpfsClient;
//! use std::io::Cursor;
//!
//! #[actix_rt::main]
//! async fn main() {
//!     let client = IpfsClient::default();
//!     let data = Cursor::new("Hello World!");
//!
//!     match client.add(data).await {
//!         Ok(res) => println!("{}", res.hash),
//!         Err(e) => eprintln!("error adding file: {}", e)
//!     }
//! }
//! ```
//!
//! ### Reading a file from IPFS
//!
//! #### With Hyper
//!
//! ```no_run
//! use futures::TryStreamExt;
//! use ipfs_api::IpfsClient;
//! use std::io::{self, Write};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = IpfsClient::default();
//!
//!     match client
//!         .get("/test/file.json")
//!         .map_ok(|chunk| chunk.to_vec())
//!         .try_concat()
//!         .await
//!     {
//!         Ok(res) => {
//!             let out = io::stdout();
//!             let mut out = out.lock();
//!
//!             out.write_all(&res).unwrap();
//!         }
//!         Err(e) => eprintln!("error getting file: {}", e)
//!     }
//! }
//! ```
//!
//! #### With Actix
//!
//! ```no_run
//! use futures::TryStreamExt;
//! use ipfs_api::IpfsClient;
//! use std::io::{self, Write};
//!
//! #[actix_rt::main]
//! async fn main() {
//!     let client = IpfsClient::default();
//!
//!     match client
//!         .get("/test/file.json")
//!         .map_ok(|chunk| chunk.to_vec())
//!         .try_concat()
//!         .await
//!     {
//!         Ok(res) => {
//!             let out = io::stdout();
//!             let mut out = out.lock();
//!
//!             out.write_all(&res).unwrap();
//!         }
//!         Err(e) => eprintln!("error getting file: {}", e)
//!     }
//! }
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
//! $ cargo run --example add_file
//! ```
//!

#[cfg(feature = "actix")]
extern crate actix_multipart_rfc7578 as actix_multipart;
#[cfg(feature = "actix")]
#[macro_use]
extern crate derive_more;

#[cfg(feature = "hyper")]
extern crate hyper_multipart_rfc7578 as hyper_multipart;
#[cfg(feature = "hyper")]
#[macro_use]
extern crate failure;

extern crate serde;

pub use crate::client::{IpfsClient, TryFromUri};
pub use crate::request::{KeyType, Logger, LoggingLevel, ObjectTemplate};

mod client;
mod header;
mod read;
mod request;
pub mod response;

#[cfg(feature = "actix")]
use actix_http::{encoding, Payload, PayloadStream};
#[cfg(feature = "hyper")]
use hyper::{self, client::HttpConnector};
#[cfg(feature = "hyper")]
use hyper_tls::HttpsConnector;

#[cfg(feature = "actix")]
pub(crate) type Request = awc::SendClientRequest;
#[cfg(feature = "hyper")]
pub(crate) type Request = http::Request<hyper::Body>;

#[cfg(feature = "actix")]
pub(crate) type Response = awc::ClientResponse<encoding::Decoder<Payload<PayloadStream>>>;
#[cfg(feature = "hyper")]
pub(crate) type Response = http::Response<hyper::Body>;

#[cfg(feature = "actix")]
pub(crate) type Client = awc::Client;
#[cfg(feature = "hyper")]
pub(crate) type Client = hyper::client::Client<HttpsConnector<HttpConnector>, hyper::Body>;
