extern crate futures;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_urlencoded;
extern crate tokio_core;


pub use client::IpfsClient;


pub mod request;
pub mod response;
mod client;
