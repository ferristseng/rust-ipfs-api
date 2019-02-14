# ipfs-api

[![Travis](https://img.shields.io/travis/ferristseng/rust-ipfs-api.svg)](https://travis-ci.org/ferristseng/rust-ipfs-api)
[![Crates.io](https://img.shields.io/crates/v/ipfs-api.svg)](https://crates.io/crates/ipfs-api)
[![Docs.rs](https://docs.rs/ipfs-api/badge.svg)](https://docs.rs/ipfs-api/)

Rust library for connecting to the IPFS HTTP API using tokio.

### Usage

```toml
[dependencies]
ipfs-api = "0.5.1"
```

You can use `actix-web` as a backend instead of `hyper`.

```toml
[dependencies]
ipfs-api = { version = "0.5.1", features = ["actix"], default-features = false }
```

### Examples

#### Writing a file to IPFS

##### With Hyper

```rust
#
use hyper::rt::Future;
use ipfs_api::IpfsClient;
use std::io::Cursor;

let client = IpfsClient::default();
let data = Cursor::new("Hello World!");

let req = client
    .add(data)
    .map(|res| {
        println!("{}", res.hash);
    })
    .map_err(|e| eprintln!("{}", e));

hyper::rt::run(req);
```

##### With Actix

```rust
#
use futures::future::Future;
use ipfs_api::IpfsClient;
use std::io::Cursor;

let client = IpfsClient::default();
let data = Cursor::new("Hello World!");

let req = client
    .add(data)
    .map(|res| {
        println!("{}", res.hash);
    })
    .map_err(|e| eprintln!("{}", e));

actix_web::actix::run(|| {
    req.then(|_| {
        actix_web::actix::System::current().stop();
        Ok(())
    })
});
```

#### Reading a file from IPFS

##### With Hyper

```rust
#
use futures::{Future, Stream};
use ipfs_api::IpfsClient;
use std::io::{self, Write};

let client = IpfsClient::default();

let req = client
    .get("/test/file.json")
    .concat2()
    .map(|res| {
        let out = io::stdout();
        let mut out = out.lock();

        out.write_all(&res).unwrap();
    })
    .map_err(|e| eprintln!("{}", e));

hyper::rt::run(req);
```

##### With Actix

```rust
#
use futures::{Future, Stream};
use ipfs_api::IpfsClient;
use std::io::{self, Write};

let client = IpfsClient::default();

let req = client
    .get("/test/file.json")
    .concat2()
    .map(|res| {
        let out = io::stdout();
        let mut out = out.lock();

        out.write_all(&res).unwrap();
    })
    .map_err(|e| eprintln!("{}", e));

actix_web::actix::run(|| {
    req.then(|_| {
        actix_web::actix::System::current().stop();
        Ok(())
    })
});
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
$ cargo run -p ipfs-api --example add_file
```

To run an example with the `actix-web` backend, use:

```sh
$ cargo run -p ipfs-api --features actix --no-default-features --example add_file
```


## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
