# ipfs-api

[![Travis](https://img.shields.io/travis/ferristseng/rust-ipfs-api.svg)](https://travis-ci.org/ferristseng/rust-ipfs-api)
[![Crates.io](https://img.shields.io/crates/v/ipfs-api.svg)](https://crates.io/crates/ipfs-api)
[![Docs.rs](https://docs.rs/ipfs-api/badge.svg)](https://docs.rs/ipfs-api/)

Rust library for connecting to the IPFS HTTP API using tokio.

### Usage

```toml
[dependencies]
ipfs-api = "0.5.0-alpha1"
```

### Examples

Write a file to IPFS:

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

Read a file from IPFS:

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

There are also a bunch of examples included in the project, which
I used for testing

You can run any of the examples with cargo:

```sh
$ cargo run -p ipfs-api --example add_file
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
