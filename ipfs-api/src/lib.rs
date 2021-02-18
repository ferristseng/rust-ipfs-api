// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

#![recursion_limit = "128"]

//! Rust library for connecting to the IPFS HTTP API using Hyper/Actix.
//!
//! ## Usage
//!
//! ```toml
//! [dependencies]
//! ipfs-api = "0.11.0"
//! ```
//! ### Feature Flags
//!
//! You can use `actix-web` as a backend instead of `hyper`.
//!
//! ```toml
//! [dependencies]
//! ipfs-api = { version = "0.11.0", features = ["with-actix"], default-features = false }
//! ```
//!
//! You also have the option of using [`rustls`](https://crates.io/crates/rustls)
//! instead of native tls:
//!
//! ```toml
//! [dependencies]
//! ipfs-api = { version = "0.11.0", features = ["with-hyper-rustls"], default-features = false }
//! ```
//!
//! To enable the builder pattern (default) use the `with-builder` feature:
//!
//! ```toml
//! [dependencies]
//! ipfs-api = { version = "0.11.0", features = ["with-hyper-rustls", "with-builder"], default-features = false }
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

#[cfg(feature = "with-actix")]
#[macro_use]
extern crate derive_more;

#[cfg(feature = "with-hyper")]
#[macro_use]
extern crate failure;

#[cfg(feature = "with-builder")]
#[macro_use]
extern crate typed_builder;

extern crate serde;

pub use crate::client::{IpfsClient, TryFromUri};
pub use crate::request::{KeyType, Logger, LoggingLevel, ObjectTemplate};

mod client;
mod header;
mod read;
pub mod request;
pub mod response;

// --- Hyper Connectors ---

#[cfg(all(feature = "with-hyper-tls", not(feature = "with-hyper-rustls")))]
type HyperConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;
#[cfg(all(feature = "with-hyper-rustls", not(feature = "with-hyper-tls")))]
type HyperConnector = hyper_rustls::HttpsConnector<hyper::client::HttpConnector>;
#[cfg(all(
    feature = "with-hyper",
    any(
        not(any(feature = "with-hyper-tls", feature = "with-hyper-rustls")),
        all(feature = "with-hyper-rustls", feature = "with-hyper-tls"),
    )
))]
type HyperConnector = hyper::client::HttpConnector;

// --- Multipart Crates ---

#[cfg(feature = "with-actix")]
pub(crate) use actix_multipart_rfc7578::client::multipart;
#[cfg(feature = "with-hyper")]
pub(crate) use hyper_multipart_rfc7578::client::multipart;
#[cfg(feature = "with-reqwest")]
pub(crate) use reqwest::multipart;

// --- Request Types ---

#[cfg(feature = "with-actix")]
pub(crate) type Request = awc::SendClientRequest;
#[cfg(feature = "with-hyper")]
pub(crate) type Request = http::Request<hyper::Body>;
#[cfg(feature = "with-reqwest")]
pub(crate) type Request = reqwest::Request;

// --- Response Types ---

#[cfg(feature = "with-actix")]
pub(crate) type Response = awc::ClientResponse<
    actix_http::encoding::Decoder<actix_http::Payload<actix_http::PayloadStream>>,
>;
#[cfg(feature = "with-hyper")]
pub(crate) type Response = http::Response<hyper::Body>;
#[cfg(feature = "with-reqwest")]
pub(crate) type Response = reqwest::Response;

// --- Client Types ----

#[cfg(feature = "with-actix")]
pub(crate) type Client = awc::Client;
#[cfg(feature = "with-hyper")]
pub(crate) type Client = hyper::client::Client<HyperConnector, hyper::Body>;
#[cfg(feature = "with-reqwest")]
pub(crate) type Client = reqwest::Client;

// --- Validations ---

#[cfg(all(feature = "with-hyper-rustls", feature = "with-hyper-tls"))]
compile_error!("Pick only one of the features: hyper-tls, hyper-rustls");

#[cfg(not(any(
    feature = "with-actix",
    feature = "with-hyper",
    feature = "with-reqwest"
)))]
compile_error!("Pick exactly one of these features: with-hyper, with-actix, with-reqwest");
