// Copyright 2022 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use http::uri::{Builder, InvalidUri, PathAndQuery, Scheme, Uri};
use multiaddr::{Multiaddr, Protocol};
use std::{
    fs,
    net::{SocketAddr, SocketAddrV4, SocketAddrV6},
    str::FromStr,
};

const VERSION_PATH_V0: &str = "/api/v0";

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
    fn from_host_and_port(scheme: Scheme, host: &str, port: u16) -> Result<Self, http::Error> {
        let authority = format!("{}:{}", host, port);
        let builder = Builder::new().scheme(scheme).authority(&authority[..]);

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

    /// Creates a new client from a multiaddr.
    ///
    fn from_multiaddr(multiaddr: Multiaddr) -> Result<Self, multiaddr::Error> {
        let mut scheme: Option<Scheme> = None;
        let mut port: Option<u16> = None;

        for addr_component in multiaddr.iter() {
            match addr_component {
                Protocol::Tcp(tcpport) => port = Some(tcpport),
                Protocol::Http => scheme = Some(Scheme::HTTP),
                Protocol::Https => scheme = Some(Scheme::HTTPS),
                _ => (),
            }
        }

        let scheme = scheme.unwrap_or(Scheme::HTTP);

        if let Some(port) = port {
            for addr_component in multiaddr.iter() {
                match addr_component {
                    Protocol::Tcp(_) | Protocol::Http | Protocol::Https => (),
                    Protocol::Ip4(v4addr) => {
                        return Ok(Self::from_ipv4(scheme, SocketAddrV4::new(v4addr, port)).unwrap())
                    }
                    Protocol::Ip6(v6addr) => {
                        return Ok(
                            Self::from_ipv6(scheme, SocketAddrV6::new(v6addr, port, 0, 0)).unwrap(),
                        )
                    }
                    Protocol::Dns(ref hostname) => {
                        return Ok(Self::from_host_and_port(scheme, hostname, port).unwrap())
                    }
                    Protocol::Dns4(ref v4host) => {
                        return Ok(Self::from_host_and_port(scheme, v4host, port).unwrap())
                    }
                    Protocol::Dns6(ref v6host) => {
                        return Ok(Self::from_host_and_port(scheme, v6host, port).unwrap())
                    }
                    _ => {
                        return Err(multiaddr::Error::InvalidMultiaddr);
                    }
                }
            }
        }

        Err(multiaddr::Error::InvalidMultiaddr)
    }

    /// Creates a new client from a multiaddr.
    ///
    fn from_multiaddr_str(multiaddr: &str) -> Result<Self, multiaddr::Error> {
        multiaddr::from_url(multiaddr)
            .map_err(|e| multiaddr::Error::ParsingError(Box::new(e)))
            .or_else(|_| Multiaddr::from_str(multiaddr))
            .and_then(Self::from_multiaddr)
    }

    /// Creates a new client connected to the endpoint specified in ~/.ipfs/api.
    ///
    #[inline]
    fn from_ipfs_config() -> Option<Self> {
        dirs::home_dir()
            .map(|home_dir| home_dir.join(".ipfs").join("api"))
            .and_then(|multiaddr_path| fs::read_to_string(&multiaddr_path).ok())
            .and_then(|multiaddr_str| Self::from_multiaddr_str(&multiaddr_str).ok())
    }
}

#[cfg(test)]
mod tests {
    use crate::TryFromUri;
    use http::uri::{Scheme, Uri};

    #[derive(Debug)]
    struct StringWrapper(String);

    impl TryFromUri for StringWrapper {
        fn build_with_base_uri(uri: Uri) -> Self {
            StringWrapper(uri.to_string())
        }
    }

    macro_rules! test_from_value_fn_ok {
        ([$method: path]: $($f: ident ($($args: expr),+) => $output: expr),+) => {
            $(
                #[test]
                fn $f() {
                    let result: Result<StringWrapper, _> = $method($($args),+);

                    assert!(
                        result.is_ok(),
                        "should be ok but failed with error: {:?}", result.unwrap_err()
                    );

                    let StringWrapper(result) = result.unwrap();

                    assert!(
                        result == $output,
                        "got: ({}) expected: ({})", result, $output
                    );
                }
            )+
        };
    }

    test_from_value_fn_ok!(
        [TryFromUri::from_str]:
        test_from_str_0_ok ("http://localhost:5001") => "http://localhost:5001/api/v0",
        test_from_str_1_ok ("https://ipfs.io:9001") => "https://ipfs.io:9001/api/v0"
    );

    test_from_value_fn_ok!(
        [TryFromUri::from_host_and_port]:
        test_from_host_and_port_0_ok (Scheme::HTTP, "localhost", 5001) => "http://localhost:5001/api/v0",
        test_from_host_and_port_1_ok (Scheme::HTTP, "ipfs.io", 9001) => "http://ipfs.io:9001/api/v0"
    );

    test_from_value_fn_ok!(
        [TryFromUri::from_multiaddr_str]:
        test_from_multiaddr_str_0_ok ("http://localhost:5001/") => "http://localhost:5001/api/v0",
        test_from_multiaddr_str_1_ok ("https://ipfs.io:9001/") => "https://ipfs.io:9001/api/v0",
        test_from_multiaddr_str_2_ok ("/ip4/127.0.0.1/tcp/5001/http") => "http://127.0.0.1:5001/api/v0",
        test_from_multiaddr_str_3_ok ("/ip6/0:0:0:0:0:0:0:0/tcp/5001/http") => "http://[::]:5001/api/v0"
    );
}
