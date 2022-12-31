[![Workflow Status](https://github.com/ferristseng/rust-ipfs-api/workflows/Rust/badge.svg)](https://github.com/ferristseng/rust-ipfs-api/actions?query=workflow%3A%22Rust%22)
![Maintenance](https://img.shields.io/badge/maintenance-deprecated-red.svg)

# ipfs-api

### Components

| Name                    | Documentation                                    | Crate                                               |
| ----------------------- | ------------------------------------------------ | --------------------------------------------------- |
| ipfs-api-prelude        | [![Docs][prelude docs badge]][prelude docs link] | [![Crate][prelude crate badge]][prelude crate link] |
| ipfs-api-backend-actix  | [![Docs][actix docs badge]][actix docs link]     | [![Crate][actix crate badge]][actix crate link]     |
| ipfs-api-backend-hyper  | [![Docs][hyper docs badge]][hyper docs link]     | [![Crate][hyper crate badge]][hyper crate link]     |
| ipfs-api (deprecated)   | [![Docs][old docs badge]][old docs link]         | [![Crate][old crate badge]][old crate link]         |

Rust library for connecting to the IPFS HTTP API using Hyper/Actix.

### Usage

#### Using Hyper

To use the Hyper backend, declare:

```toml
[dependencies]
ipfs-api-backend-hyper = "0.6"
```

You can specify either `with-hyper-rustls` or `with-hyper-tls` (mutually exclusive) feature for TLS support.

#### Using Actix

To use the Actix backend, declare:

```toml
[dependencies]
ipfs-api-backend-actix = "0.7"
```

#### Builder Pattern

With either the Hyper or Actix backend, you can specify the `with-builder` feature to enable a builder pattern to use when building requests.

### Usage (DEPRECATED)

```toml
[dependencies]
ipfs-api = "0.17.0"
```

#### Feature Flags (DEPRECATED)

You can use `actix-web` as a backend instead of `hyper`.

```toml
[dependencies]
ipfs-api = { version = "0.17.0", features = ["with-actix"], default-features = false }
```

You also have the option of using [`rustls`](https://crates.io/crates/rustls)
instead of native tls:

```toml
[dependencies]
ipfs-api = { version = "0.17.0", features = ["with-hyper-rustls"], default-features = false }
```

To enable the builder pattern (default) use the `with-builder` feature:

```toml
[dependencies]
ipfs-api = { version = "0.17.0", features = ["with-hyper-rustls", "with-builder"], default-features = false }
```

### Examples

#### Writing a file to IPFS

##### With Hyper

```rust
use ipfs_api::{IpfsApi, IpfsClient};
use std::io::Cursor;

#[tokio::main]
async fn main() {
    let client = IpfsClient::default();
    let data = Cursor::new("Hello World!");

    match client.add(data).await {
        Ok(res) => println!("{}", res.hash),
        Err(e) => eprintln!("error adding file: {}", e)
    }
}
```

##### With Actix

```rust
use ipfs_api::{IpfsApi, IpfsClient};
use std::io::Cursor;

#[actix_rt::main]
async fn main() {
    let client = IpfsClient::default();
    let data = Cursor::new("Hello World!");

    match client.add(data).await {
        Ok(res) => println!("{}", res.hash),
        Err(e) => eprintln!("error adding file: {}", e)
    }
}
```

#### Reading a file from IPFS

##### With Hyper

```rust
use futures::TryStreamExt;
use ipfs_api::{IpfsApi, IpfsClient};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let client = IpfsClient::default();

    match client
        .get("/test/file.json")
        .map_ok(|chunk| chunk.to_vec())
        .try_concat()
        .await
    {
        Ok(res) => {
            let out = io::stdout();
            let mut out = out.lock();

            out.write_all(&res).unwrap();
        }
        Err(e) => eprintln!("error getting file: {}", e)
    }
}
```

##### With Actix

```rust
use futures::TryStreamExt;
use ipfs_api::{IpfsApi, IpfsClient};
use std::io::{self, Write};

#[actix_rt::main]
async fn main() {
    let client = IpfsClient::default();

    match client
        .get("/test/file.json")
        .map_ok(|chunk| chunk.to_vec())
        .try_concat()
        .await
    {
        Ok(res) => {
            let out = io::stdout();
            let mut out = out.lock();

            out.write_all(&res).unwrap();
        }
        Err(e) => eprintln!("error getting file: {}", e)
    }
}
```

#### Additional Examples

There are also a bunch of examples included in the project, which
I used for testing

For a list of examples, run:

```sh
$ cargo run --example
```

You can run any of the examples with cargo:

```sh
$ cargo run --example add_file
```


## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[prelude docs badge]: https://img.shields.io/docsrs/ipfs-api-prelude/latest "ipfs-api-prelude documentation"
[prelude docs link]: https://docs.rs/ipfs-api-prelude
[prelude crate badge]: https://img.shields.io/crates/v/ipfs-api-prelude.svg "ipfs-api-prelude crates.io"
[prelude crate link]: https://crates.io/crates/ipfs-api-prelude
[actix docs badge]: https://docs.rs/ipfs-api-backend-actix/badge.svg "ipfs-api-backend-actix documentation"
[actix docs link]: https://docs.rs/ipfs-api-backend-actix
[actix crate badge]: https://img.shields.io/crates/v/ipfs-api-backend-actix.svg "ipfs-api-backend-actix crates.io"
[actix crate link]: https://crates.io/crates/ipfs-api-backend-actix
[hyper docs badge]: https://docs.rs/ipfs-api-backend-hyper/badge.svg "ipfs-api-backend-hyper documentation"
[hyper docs link]: https://docs.rs/ipfs-api-backend-hyper
[hyper crate badge]: https://img.shields.io/crates/v/ipfs-api-backend-hyper.svg "ipfs-api-backend-hyper crates.io"
[hyper crate link]: https://crates.io/crates/ipfs-api-backend-hyper
[old docs badge]: https://docs.rs/ipfs-api/badge.svg "ipfs-api (deprecated) documentation"
[old docs link]: https://docs.rs/ipfs-api
[old crate badge]: https://img.shields.io/crates/v/ipfs-api.svg "ipfs-api (deprecated) crates.io"
[old crate link]: https://crates.io/crates/ipfs-api
