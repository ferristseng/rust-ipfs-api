# ipfs-api

[![Travis](https://img.shields.io/travis/ferristseng/rust-ipfs-api.svg)](https://travis-ci.org/ferristseng/rust-ipfs-api)
[![Crates.io](https://img.shields.io/crates/v/ipfs-api.svg)](https://crates.io/crates/ipfs-api)
[![Docs.rs](https://docs.rs/ipfs-api/badge.svg)](https://docs.rs/ipfs-api/)

Rust library for connecting to the IPFS HTTP API using tokio.

### Usage

```toml
[dependencies]
ipfs-api = "0.9.0"
```

You can use `actix-web` as a backend instead of `hyper`.

```toml
[dependencies]
ipfs-api = { version = "0.9.0", features = ["with-actix"], default-features = false }
```

### Examples

#### Writing a file to IPFS

##### With Hyper

```rust
use ipfs_api::IpfsClient;
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
use ipfs_api::IpfsClient;
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
use ipfs_api::IpfsClient;
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
use ipfs_api::IpfsClient;
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
