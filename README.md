## Rust IPFS API Library

```toml
[dependencies]
ipfs-api = "0.4.0"
```

### Goals

  * Provide a full implementation of the HTTP API specification described here: https://ipfs.io/docs/api/.
  * Write idiomatic rust, and make use of rust's memory safety features.
  * Provide support for `go-ipfs 0.4.*`, with possible backwards compatibility features.
  * Feature parity with the `go-ipfs` cli.
  * Provide cross platform support for Linux, OSX, and Windows.

#### Maybe (?)

  * Add integration tests for the `go-ipfs` implementation, and `js-ipfs` implementation of the ipfs spec.
  * Explore a higher level API for interacting with IPFS.
  * File system abstraction

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
