// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

//! Rust library for connecting to the IPFS HTTP API using Hyper/Actix.
//!
//! ## Usage
//!
//! ### Using Hyper
//!
//! To use the Hyper backend, declare:
//!
//! ```toml
//! [dependencies]
//! ipfs-api-backend-hyper = "0.6"
//! ```
//!
//! You can specify either `with-hyper-rustls` or `with-hyper-tls` (mutually exclusive) feature for TLS support.
//!
//! ### Using Actix
//!
//! To use the Actix backend, declare:
//!
//! ```toml
//! [dependencies]
//! ipfs-api-backend-actix = "0.7"
//! ```
//!
//! ### Builder Pattern
//!
//! With either the Hyper or Actix backend, you can specify the `with-builder` feature to enable a builder pattern to use when building requests.
//!
//! ## Usage (DEPRECATED)
//!
//! ```toml
//! [dependencies]
//! ipfs-api = "0.17.0"
//! ```
//!
//! ### Feature Flags (DEPRECATED)
//!
//! You can use `actix-web` as a backend instead of `hyper`.
//!
//! ```toml
//! [dependencies]
//! ipfs-api = { version = "0.17.0", features = ["with-actix"], default-features = false }
//! ```
//!
//! You also have the option of using [`rustls`](https://crates.io/crates/rustls)
//! instead of native tls:
//!
//! ```toml
//! [dependencies]
//! ipfs-api = { version = "0.17.0", features = ["with-hyper-rustls"], default-features = false }
//! ```
//!
//! To enable the builder pattern (default) use the `with-builder` feature:
//!
//! ```toml
//! [dependencies]
//! ipfs-api = { version = "0.17.0", features = ["with-hyper-rustls", "with-builder"], default-features = false }
//! ```
//!
//! ## Examples
//!
//! ### Writing a file to IPFS
//!
//! #### With Hyper
//!
//! ```no_run
//! use ipfs_api::{IpfsApi, IpfsClient};
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
//! use ipfs_api::{IpfsApi, IpfsClient};
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
//! use ipfs_api::{IpfsApi, IpfsClient};
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
//! use ipfs_api::{IpfsApi, IpfsClient};
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

#[cfg(feature = "with-hyper")]
pub use ipfs_api_backend_hyper::*;

#[cfg(feature = "with-actix")]
pub use ipfs_api_backend_actix::*;

#[cfg(not(any(feature = "with-actix", feature = "with-hyper")))]
compile_error!("Pick exactly one of these features: with-hyper, with-actix");
