// Copyright 2022 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Define which IPFS (Kubo) Docker images the Rust library will be tested against.
//!
//! # Examples
//!
//! ```rust
//! use ipfs_api_versions::test_current_image;
//!
//! #[test_current_image]
//! fn test_foo(image_name: &str, image_tag: &str) {
//!     // ...test implementation to run against only the latest image
//! }
//! ```
//!
//! ```rust
//! use ipfs_api_versions::test_supported_images;
//!
//! #[test_supported_images]
//! fn test_bar(image_name: &str, image_tag: &str) {
//!     // ...test implementation to run against all supported images
//! }
//! ```

use proc_macro::TokenStream as CompilerTokenStream;
use quote::{quote, quote_spanned};

/// Docker images of IPFS daemon versions supported by this library, in ascending order.
fn supported() -> Vec<(String, String)> {
    let source = [
        ("ipfs/go-ipfs", "v0.7.0"),
        ("ipfs/go-ipfs", "v0.8.0"),
        ("ipfs/go-ipfs", "v0.9.1"),
        ("ipfs/go-ipfs", "v0.10.0"),
        ("ipfs/go-ipfs", "v0.11.1"),
        ("ipfs/go-ipfs", "v0.12.2"),
        ("ipfs/go-ipfs", "v0.13.0"),
        ("ipfs/kubo", "v0.14.0"),
        ("ipfs/kubo", "v0.15.0"),
        ("ipfs/kubo", "v0.16.0"),
        ("ipfs/kubo", "v0.17.0"),
        ("ipfs/kubo", "v0.18.0"),
        ("ipfs/kubo", "v0.19.0"),
        ("ipfs/kubo", "v0.20.0"),
        ("ipfs/kubo", "v0.21.0"),
        ("ipfs/kubo", "v0.22.0"),
    ];

    source
        .into_iter()
        .map(|(i, t)| (i.into(), t.into()))
        .collect()
}

/// Docker image of most recent supported IPFS daemon.
fn current() -> (String, String) {
    supported().into_iter().last().unwrap()
}

fn image_test_case(image_name: &str, image_tag: &str) -> proc_macro2::TokenStream {
    quote! {
        #[test_case::test_case(#image_name, #image_tag)]
    }
}

fn unexpected_meta(meta: CompilerTokenStream) -> Option<CompilerTokenStream> {
    let m2: proc_macro2::TokenStream = meta.into();

    if let Some(m) = m2.into_iter().next() {
        let result = quote_spanned! { m.span() =>
            compile_error!("Macro does not expect any arguments.");
        };

        Some(result.into())
    } else {
        None
    }
}

#[proc_macro_attribute]
pub fn test_current_image(
    meta: CompilerTokenStream,
    input: CompilerTokenStream,
) -> CompilerTokenStream {
    if let Some(err) = unexpected_meta(meta) {
        err
    } else {
        let (image_name, image_tag) = current();

        let tokens = vec![image_test_case(&image_name, &image_tag), input.into()];

        let result = quote! {
            #(#tokens)*
        };

        result.into()
    }
}

#[proc_macro_attribute]
pub fn test_supported_images(
    meta: CompilerTokenStream,
    input: CompilerTokenStream,
) -> CompilerTokenStream {
    if let Some(err) = unexpected_meta(meta) {
        err
    } else {
        let mut tokens: Vec<_> = supported()
            .iter()
            .map(|(image_name, image_tag)| {
                quote! {
                    #[test_case::test_case(#image_name, #image_tag)]
                }
            })
            .collect();

        tokens.push(input.into());

        let result = quote! {
            #(#tokens)*
        };

        result.into()
    }
}
