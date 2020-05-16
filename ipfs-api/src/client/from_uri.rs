// Copyright 2020 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
use http::uri::{Builder, InvalidUri, PathAndQuery, Scheme, Uri};
use parity_multiaddr::{Multiaddr, Protocol};
use std::{
    fs,
    net::{IpAddr, SocketAddr, SocketAddrV4, SocketAddrV6},
};

const VERSION_PATH_V0: &'static str = "/api/v0";

/// Builds the base url path for the Ipfs api.
///
fn build_base_path(builder: Builder) -> Result<Uri, http::Error> {
    builder.path_and_query(VERSION_PATH_V0).build()
}

pub trait TryFromUri: Sized {
    /// Builds a new client from a base URI to the IPFS API.
    ///
    fn build_with_base_uri(uri: Uri) -> Self;

    /// Creates a new client from a str.
    ///
    /// Note: This constructor will overwrite the path/query part of the URI.
    ///
    fn from_str(uri: &str) -> Result<Self, InvalidUri> {
        let uri: Uri = uri.parse()?;
        let mut parts = uri.into_parts();

        parts.path_and_query = Some(PathAndQuery::from_static(VERSION_PATH_V0));

        Ok(Self::build_with_base_uri(Uri::from_parts(parts).unwrap()))
    }

    /// Creates a new client from a host name and port.
    ///
    fn from_host_and_port(host: &str, port: u16) -> Result<Self, http::Error> {
        let authority = format!("{}:{}", host, port);
        let builder = Builder::new()
            .scheme(Scheme::HTTP)
            .authority(&authority[..]);

        build_base_path(builder).map(Self::build_with_base_uri)
    }

    /// Creates a new client from an IPV4 address and port number.
    ///
    fn from_ipv4(scheme: Scheme, addr: SocketAddrV4) -> Result<Self, http::Error> {
        let authority = format!("{}", addr);
        let builder = Builder::new().scheme(scheme).authority(&authority[..]);

        build_base_path(builder).map(Self::build_with_base_uri)
    }

    /// Creates a new client from an IPV6 addr and port number.
    ///
    fn from_ipv6(scheme: Scheme, addr: SocketAddrV6) -> Result<Self, http::Error> {
        let authority = format!("{}", addr);
        let builder = Builder::new().scheme(scheme).authority(&authority[..]);

        build_base_path(builder).map(Self::build_with_base_uri)
    }

    /// Creates a new client from an IP address and port number.
    ///
    fn from_socket(scheme: Scheme, socket_addr: SocketAddr) -> Result<Self, http::Error> {
        match socket_addr {
            SocketAddr::V4(addr) => Self::from_ipv4(scheme, addr),
            SocketAddr::V6(addr) => Self::from_ipv6(scheme, addr),
        }
    }

    /// Creates a new client connected to the endpoint specified in ~/.ipfs/api.
    ///
    fn from_multiaddr(multiaddr: Multiaddr) -> Option<Self> {
        let mut addr: Option<IpAddr> = None;
        let mut port: Option<u16> = None;

        for addr_component in multiaddr.iter() {
            match addr_component {
                Protocol::Ip4(v4addr) => addr = Some(v4addr.into()),
                Protocol::Ip6(v6addr) => addr = Some(v6addr.into()),
                Protocol::Tcp(tcpport) => port = Some(tcpport),
                _ => {
                    return None;
                }
            }
        }

        if let (Some(addr), Some(port)) = (addr, port) {
            Some(Self::from_socket(Scheme::HTTP, SocketAddr::new(addr, port)).unwrap())
        } else {
            None
        }
    }

    /// Creates a new client connected to the endpoint specified in ~/.ipfs/api.
    ///
    #[inline]
    fn from_ipfs_config() -> Option<Self> {
        dirs::home_dir()
            .map(|home_dir| home_dir.join(".ipfs").join("api"))
            .and_then(|multiaddr_path| fs::read_to_string(&multiaddr_path).ok())
            .and_then(|multiaddr_str| parity_multiaddr::from_url(&multiaddr_str).ok())
            .and_then(Self::from_multiaddr)
    }
}
