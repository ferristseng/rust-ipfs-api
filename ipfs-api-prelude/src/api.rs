// Copyright 2021 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::{read::LineDecoder, request, response, Backend, BoxStream};
use async_trait::async_trait;
use bytes::Bytes;
use common_multipart_rfc7578::client::multipart;
use futures::{future, AsyncRead, FutureExt, TryStreamExt};
use std::{
    fs::File,
    io::{Cursor, Read},
    path::{Path, PathBuf},
};

const FILE_DESCRIPTOR_LIMIT: usize = 127;

// Implements a call to the IPFS that returns a streaming body response.
// Implementing this in a macro is necessary because the Rust compiler
// can't reason about the lifetime of the request instance properly. It
// thinks that the request needs to live as long as the returned stream,
// but in reality, the request instance is only used to build the Hyper
// or Actix request.
//
macro_rules! impl_stream_api_response {
    (($self:ident, $req:expr, $form:expr) => $call:ident) => {
        impl_stream_api_response! {
            ($self, $req, $form) |req| => { $self.$call(req) }
        }
    };
    (($self:ident, $req:expr, $form:expr) |$var:ident| => $impl:block) => {
        match $self.build_base_request($req, $form) {
            Ok($var) => $impl,
            Err(e) => Box::new(future::err(e).into_stream()),
        }
    };
}

#[cfg_attr(feature = "with-send-sync", async_trait)]
#[cfg_attr(not(feature = "with-send-sync"), async_trait(?Send))]
pub trait IpfsApi: Backend {
    /// Add file to Ipfs.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    /// use std::io::Cursor;
    ///
    /// let client = IpfsClient::default();
    /// let data = Cursor::new("Hello World!");
    /// let res = client.add(data);
    /// ```
    ///
    async fn add<R>(&self, data: R) -> Result<response::AddResponse, Self::Error>
    where
        R: 'static + Read + Send + Sync + Unpin,
    {
        self.add_with_options(data, request::Add::default()).await
    }

    /// Add AsyncRead stream to Ipfs.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    /// use std::io::Cursor;
    ///
    /// let client = IpfsClient::default();
    /// let data = b"Hello World!";
    /// let res = client.add_async(&data[..]);
    /// ```
    ///
    async fn add_async<R>(&self, data: R) -> Result<response::AddResponse, Self::Error>
    where
        R: 'static + AsyncRead + Send + Sync + Unpin,
    {
        self.add_async_with_options(data, request::Add::default())
            .await
    }

    /// Add a file to IPFS with options.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::{IpfsApi, IpfsClient};
    /// use std::io::Cursor;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let data = Cursor::new("Hello World!");
    /// #[cfg(feature = "with-builder")]
    /// let add = ipfs_api::request::Add::builder()
    ///     .chunker("rabin-512-1024-2048")
    ///     .build();
    /// #[cfg(not(feature = "with-builder"))]
    /// let add = ipfs_api::request::Add {
    ///     chunker: Some("rabin-512-1024-2048"),
    ///     ..Default::default()
    /// };
    /// let req = client.add_with_options(data, add);
    /// # }
    /// ```
    ///
    async fn add_with_options<R>(
        &self,
        data: R,
        add: request::Add<'_>,
    ) -> Result<response::AddResponse, Self::Error>
    where
        R: 'static + Read + Send + Sync + Unpin,
    {
        let mut form = multipart::Form::default();

        form.add_reader("path", data);

        self.request(add, Some(form)).await
    }

    /// Add AsyncRead stream to IPFS with options.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let data = b"Hello World!";
    /// #[cfg(feature = "with-builder")]
    /// let add = ipfs_api::request::Add::builder()
    ///     .chunker("rabin-512-1024-2048")
    ///     .build();
    /// #[cfg(not(feature = "with-builder"))]
    /// let add = ipfs_api::request::Add {
    ///     chunker: Some("rabin-512-1024-2048"),
    ///     ..Default::default()
    /// };
    /// let req = client.add_async_with_options(&data[..], add);
    /// # }
    /// ```
    ///
    async fn add_async_with_options<R>(
        &self,
        data: R,
        add: request::Add<'_>,
    ) -> Result<response::AddResponse, Self::Error>
    where
        R: 'static + AsyncRead + Send + Sync + Unpin,
    {
        let mut form = multipart::Form::default();

        form.add_async_reader("path", data);

        self.request(add, Some(form)).await
    }

    /// Add files using multipart::Form
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient, Form};
    /// use common_multipart_rfc7578::client::multipart;
    /// use std::io::Cursor;
    ///
    /// #[cfg(feature = "with-builder")]
    /// let add = ipfs_api::request::Add::builder()
    ///     .wrap_with_directory(true)
    ///     .build();
    /// #[cfg(not(feature = "with-builder"))]
    /// let add = ipfs_api::request::Add {
    ///     wrap_with_directory: Some(true),
    ///     ..Default::default()
    /// };
    ///
    /// let mut form = Form::default();
    /// form.add_reader_file("path", Cursor::new(Vec::new()), "file.txt");
    ///
    /// let client = IpfsClient::default();
    /// let res = client.add_with_form(form, add);
    /// ```
    async fn add_with_form<F>(
        &self,
        form: F,
        add: request::Add<'_>,
    ) -> Result<Vec<response::AddResponse>, Self::Error>
    where
        F: Into<multipart::Form<'static>> + Send,
    {
        let req = self.build_base_request(add, Some(form.into()))?;
        self.request_stream_json(req).try_collect().await
    }

    /// Add a path to Ipfs. Can be a file or directory.
    /// A hard limit of 128 open file descriptors is set such
    /// that any small additional files are stored in-memory.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let path = "./src";
    /// let res = client.add_path(path);
    /// ```
    ///
    async fn add_path<P>(&self, path: P) -> Result<Vec<response::AddResponse>, Self::Error>
    where
        P: AsRef<Path> + Send,
    {
        let prefix = path.as_ref().parent();
        let mut paths_to_add: Vec<(PathBuf, u64)> = vec![];

        for path in walkdir::WalkDir::new(path.as_ref()) {
            match path {
                Ok(entry) if entry.file_type().is_file() => {
                    let file_size = entry
                        .metadata()
                        .map(|metadata| metadata.len())
                        .map_err(|e| crate::Error::Io(e.into()))?;

                    paths_to_add.push((entry.path().to_path_buf(), file_size));
                }
                Ok(_) => (),
                Err(e) => return Err(crate::Error::Io(e.into()).into()),
            }
        }

        paths_to_add.sort_unstable_by(|(_, a), (_, b)| a.cmp(b).reverse());

        let mut it = 0;
        let mut form = multipart::Form::default();

        for (path, file_size) in paths_to_add {
            let mut file = File::open(&path).map_err(crate::Error::Io)?;
            let file_name = match prefix {
                Some(prefix) => path.strip_prefix(prefix).unwrap(),
                None => path.as_path(),
            }
            .to_string_lossy();

            if it < FILE_DESCRIPTOR_LIMIT {
                form.add_reader_file("path", file, file_name);

                it += 1;
            } else {
                let mut buf = Vec::with_capacity(file_size as usize);
                let _ = file.read_to_end(&mut buf).map_err(crate::Error::Io)?;

                form.add_reader_file("path", Cursor::new(buf), file_name);
            }
        }

        let req = self.build_base_request(request::Add::default(), Some(form))?;

        self.request_stream_json(req).try_collect().await
    }

    /// Returns the current ledger for a peer.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.bitswap_ledger("QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ");
    /// ```
    ///
    async fn bitswap_ledger(
        &self,
        peer: &str,
    ) -> Result<response::BitswapLedgerResponse, Self::Error> {
        self.request(request::BitswapLedger { peer }, None).await
    }

    /// Triggers a reprovide.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.bitswap_reprovide();
    /// ```
    ///
    async fn bitswap_reprovide(&self) -> Result<response::BitswapReprovideResponse, Self::Error> {
        self.request_empty(request::BitswapReprovide, None).await
    }

    /// Returns some stats about the bitswap agent.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.bitswap_stat();
    /// ```
    ///
    async fn bitswap_stat(&self) -> Result<response::BitswapStatResponse, Self::Error> {
        self.request(request::BitswapStat, None).await
    }

    /// Remove a given block from your wantlist.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.bitswap_unwant("QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA");
    /// ```
    ///
    async fn bitswap_unwant(
        &self,
        key: &str,
    ) -> Result<response::BitswapUnwantResponse, Self::Error> {
        self.request_empty(request::BitswapUnwant { key }, None)
            .await
    }

    /// Shows blocks on the wantlist for you or the specified peer.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.bitswap_wantlist(
    ///     Some("QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ")
    /// );
    /// ```
    ///
    async fn bitswap_wantlist(
        &self,
        peer: Option<&str>,
    ) -> Result<response::BitswapWantlistResponse, Self::Error> {
        self.request(request::BitswapWantlist { peer }, None).await
    }

    /// Gets a raw IPFS block.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use futures::TryStreamExt;
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let hash = "QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA";
    /// let res = client
    ///     .block_get(hash)
    ///     .map_ok(|chunk| chunk.to_vec())
    ///     .try_concat();
    /// ```
    ///
    fn block_get(&self, hash: &str) -> BoxStream<Bytes, Self::Error> {
        impl_stream_api_response! {
            (self, request::BlockGet { hash }, None) => request_stream_bytes
        }
    }

    /// Store input as an IPFS block.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    /// use std::io::Cursor;
    ///
    /// let client = IpfsClient::default();
    /// let data = Cursor::new("Hello World!");
    /// let res = client.block_put(data);
    /// ```
    ///
    async fn block_put<R>(&self, data: R) -> Result<response::BlockPutResponse, Self::Error>
    where
        R: 'static + Read + Send + Sync + Unpin,
    {
        self.block_put_with_options(data, request::BlockPut::default())
            .await
    }

    /// Store input as an IPFS block with options.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    /// use std::io::Cursor;
    ///
    /// #[cfg(feature = "with-builder")]
    /// let options = ipfs_api::request::BlockPut::builder()
    ///     .mhtype("sha3_384")
    ///     .mhlen(48)
    ///     .build();
    /// #[cfg(not(feature = "with-builder"))]
    /// let options = ipts_api::request::BlockPut {
    ///     mhtype: "sha3_384",
    ///     mhlen: 48,
    ///     ..Default::default()
    /// };
    /// let client = IpfsClient::default();
    /// let data = Cursor::new("Hello World!");
    /// let res = client.block_put_with_options(data, options);
    /// ```
    ///
    async fn block_put_with_options<R>(
        &self,
        data: R,
        options: request::BlockPut<'async_trait>,
    ) -> Result<response::BlockPutResponse, Self::Error>
    where
        R: 'static + Read + Send + Sync + Unpin,
    {
        let mut form = multipart::Form::default();

        form.add_reader("data", data);

        self.request(options, Some(form)).await
    }

    /// Removes an IPFS block.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.block_rm("QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA");
    /// ```
    ///
    async fn block_rm(&self, hash: &str) -> Result<response::BlockRmResponse, Self::Error> {
        self.request(request::BlockRm { hash }, None).await
    }

    /// Prints information about a raw IPFS block.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.block_stat("QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA");
    /// ```
    ///
    async fn block_stat(&self, hash: &str) -> Result<response::BlockStatResponse, Self::Error> {
        self.request(request::BlockStat { hash }, None).await
    }

    /// Add default peers to the bootstrap list.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.bootstrap_add_default();
    /// ```
    ///
    async fn bootstrap_add_default(
        &self,
    ) -> Result<response::BootstrapAddDefaultResponse, Self::Error> {
        self.request(request::BootstrapAddDefault, None).await
    }

    /// Lists peers in bootstrap list.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.bootstrap_list();
    /// ```
    ///
    async fn bootstrap_list(&self) -> Result<response::BootstrapListResponse, Self::Error> {
        self.request(request::BootstrapList, None).await
    }

    /// Removes all peers in bootstrap list.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.bootstrap_rm_all();
    /// ```
    ///
    async fn bootstrap_rm_all(&self) -> Result<response::BootstrapRmAllResponse, Self::Error> {
        self.request(request::BootstrapRmAll, None).await
    }

    /// Returns the contents of an Ipfs object.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use futures::TryStreamExt;
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let hash = "QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA";
    /// let res = client
    ///     .cat(hash)
    ///     .map_ok(|chunk| chunk.to_vec())
    ///     .try_concat();
    /// ```
    ///
    fn cat(&self, path: &str) -> BoxStream<Bytes, Self::Error> {
        let offset = None;
        let length = None;
        impl_stream_api_response! {
            (self, request::Cat { path, offset, length }, None) => request_stream_bytes
        }
    }

    /// Returns the the specified range of bytes of an Ipfs object.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use futures::TryStreamExt;
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let hash = "QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA";
    /// let offset = 0;
    /// let length = 10;
    ///
    /// let res = client
    ///     .cat_range(hash, offset, length)
    ///     .map_ok(|chunk| chunk.to_vec())
    ///     .try_concat();
    ///
    /// ```
    ///
    fn cat_range(
        &self,
        path: &str,
        _offset: usize,
        _length: usize,
    ) -> BoxStream<Bytes, Self::Error> {
        let offset = Some(_offset);
        let length = Some(_length);
        impl_stream_api_response! {
            (self, request::Cat { path, offset, length }, None) => request_stream_bytes
        }
    }

    /// List available commands that the server accepts.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.commands();
    /// ```
    ///
    async fn commands(&self) -> Result<response::CommandsResponse, Self::Error> {
        self.request(request::Commands, None).await
    }

    /// Get ipfs config strings.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.config_get_string("Identity.PeerID");
    /// ```
    ///
    async fn config_get_string(&self, key: &str) -> Result<response::ConfigResponse, Self::Error> {
        self.request(
            request::Config {
                key,
                value: None,
                boolean: None,
                stringified_json: None,
            },
            None,
        )
        .await
    }

    /// Get ipfs config booleans.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.config_get_bool("Datastore.HashOnRead");
    /// ```
    ///
    async fn config_get_bool(&self, key: &str) -> Result<response::ConfigResponse, Self::Error> {
        self.request(
            request::Config {
                key,
                value: None,
                boolean: None,
                stringified_json: None,
            },
            None,
        )
        .await
    }

    /// Get ipfs config json.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.config_get_json("Mounts");
    /// ```
    ///
    async fn config_get_json(&self, key: &str) -> Result<response::ConfigResponse, Self::Error> {
        self.request(
            request::Config {
                key,
                value: None,
                boolean: None,
                stringified_json: None,
            },
            None,
        )
        .await
    }

    /// Set ipfs config string.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.config_set_string("Routing.Type", "dht");
    /// ```
    ///
    async fn config_set_string(
        &self,
        key: &str,
        value: &str,
    ) -> Result<response::ConfigResponse, Self::Error> {
        self.request(
            request::Config {
                key,
                value: Some(value),
                boolean: None,
                stringified_json: None,
            },
            None,
        )
        .await
    }

    /// Set ipfs config boolean.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.config_set_bool("Pubsub.DisableSigning", false);
    /// ```
    ///
    async fn config_set_bool(
        &self,
        key: &str,
        value: bool,
    ) -> Result<response::ConfigResponse, Self::Error> {
        self.request(
            request::Config {
                key,
                value: Some(&value.to_string()),
                boolean: Some(true),
                stringified_json: None,
            },
            None,
        )
        .await
    }

    /// Set ipfs config json.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.config_set_json("Discovery", r#"{"MDNS":{"Enabled":true,"Interval":10}}"#);
    /// ```
    ///
    async fn config_set_json(
        &self,
        key: &str,
        value: &str,
    ) -> Result<response::ConfigResponse, Self::Error> {
        self.request(
            request::Config {
                key,
                value: Some(value),
                boolean: None,
                stringified_json: Some(true),
            },
            None,
        )
        .await
    }

    /// Opens the config file for editing (on the server).
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.config_edit();
    /// ```
    ///
    async fn config_edit(&self) -> Result<response::ConfigEditResponse, Self::Error> {
        self.request(request::ConfigEdit, None).await
    }

    /// Replace the config file.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    /// use std::io::Cursor;
    ///
    /// let client = IpfsClient::default();
    /// let config = Cursor::new("{..json..}");
    /// let res = client.config_replace(config);
    /// ```
    ///
    async fn config_replace<R>(
        &self,
        data: R,
    ) -> Result<response::ConfigReplaceResponse, Self::Error>
    where
        R: 'static + Read + Send + Sync + Unpin,
    {
        let mut form = multipart::Form::default();

        form.add_reader("file", data);

        self.request_empty(request::ConfigReplace, Some(form)).await
    }

    /// Show the current config of the server.
    ///
    /// Returns an unparsed json string, due to an unclear spec.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.config_show();
    /// ```
    ///
    async fn config_show(&self) -> Result<response::ConfigShowResponse, Self::Error> {
        self.request_string(request::ConfigShow, None).await
    }

    /// Returns information about a dag node in Ipfs.
    ///
    /// ```no_run
    /// use futures::TryStreamExt;
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let hash = "QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA";
    /// let res = client
    ///     .dag_get(hash)
    ///     .map_ok(|chunk| chunk.to_vec())
    ///     .try_concat();
    /// ```
    ///
    fn dag_get(&self, path: &str) -> BoxStream<Bytes, Self::Error> {
        self.dag_get_with_options(request::DagGet {
            path,
            ..Default::default()
        })
    }

    /// Returns information about a dag node in Ipfs with options.
    ///
    /// ```no_run
    /// use futures::TryStreamExt;
    /// use ipfs_api::{request::{DagCodec, DagGet}, IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let hash = "QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA";
    /// #[cfg(feature = "with-builder")]
    /// let options = DagGet::builder().path(hash).codec(DagCodec::Cbor).build();
    /// #[cfg(not(feature = "with-builder"))]
    /// let options = DagGet {
    ///     path: hash,
    ///     codec: DagCodec::Cbor,
    /// };
    /// client.dag_get_with_options(options)
    ///     .map_ok(|chunk| chunk.to_vec())
    ///     .try_concat();
    /// ```
    ///
    fn dag_get_with_options(&self, options: request::DagGet) -> BoxStream<Bytes, Self::Error> {
        impl_stream_api_response! {
            (self, options, None) => request_stream_bytes
        }
    }

    /// Add a DAG node to Ipfs.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    /// use std::io::Cursor;
    ///
    /// let client = IpfsClient::default();
    /// let data = Cursor::new(r#"{ "hello" : "world" }"#);
    /// let res = client.dag_put(data);
    /// ```
    ///
    async fn dag_put<R>(&self, data: R) -> Result<response::DagPutResponse, Self::Error>
    where
        R: 'static + Read + Send + Sync + Unpin,
    {
        self.dag_put_with_options(data, request::DagPut::default())
            .await
    }

    /// Add a DAG node to Ipfs with options.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    /// use std::io::Cursor;
    /// use ipfs_api::request::{DagCodec, DagPut};
    ///
    /// let client = IpfsClient::default();
    /// let data = Cursor::new(r#"{ "hello" : "world" }"#);
    /// let dag_put = DagPut::builder()
    ///     .input_codec(DagCodec::Json)
    ///     .pin(false)
    ///     .build();
    /// let res = client.dag_put_with_options(data, dag_put);
    /// ```
    ///
    async fn dag_put_with_options<'a, R>(
        &self,
        data: R,
        options: request::DagPut<'a>,
    ) -> Result<response::DagPutResponse, Self::Error>
    where
        R: 'static + Read + Send + Sync + Unpin,
    {
        let mut form = multipart::Form::default();

        form.add_reader("object data", data);

        self.request(options, Some(form)).await
    }

    // TODO /dag/resolve

    /// Query the DHT for all of the multiaddresses associated with a Peer ID.
    ///
    /// ```no_run
    /// use futures::TryStreamExt;
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let peer = "QmSoLPppuBtQSGwKDZT2M73ULpjvfd3aZ6ha4oFGL1KrGM";
    /// let res = client.dht_findpeer(peer).try_collect::<Vec<_>>();
    /// ```
    ///
    fn dht_findpeer(&self, peer: &str) -> BoxStream<response::DhtFindPeerResponse, Self::Error> {
        impl_stream_api_response! {
            (self, request::DhtFindPeer { peer }, None) => request_stream_json
        }
    }

    /// Find peers in the DHT that can provide a specific value given a key.
    ///
    /// ```no_run
    /// use futures::TryStreamExt;
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let key = "QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA";
    /// let res = client.dht_findprovs(key).try_collect::<Vec<_>>();
    /// ```
    ///
    fn dht_findprovs(&self, key: &str) -> BoxStream<response::DhtFindProvsResponse, Self::Error> {
        impl_stream_api_response! {
            (self, request::DhtFindProvs { key }, None) => request_stream_json
        }
    }

    /// Query the DHT for a given key.
    ///
    /// ```no_run
    /// use futures::TryStreamExt;
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let key = "QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA";
    /// let res = client.dht_get(key).try_collect::<Vec<_>>();
    /// ```
    ///
    fn dht_get(&self, key: &str) -> BoxStream<response::DhtGetResponse, Self::Error> {
        impl_stream_api_response! {
            (self, request::DhtGet { key }, None) => request_stream_json
        }
    }

    /// Announce to the network that you are providing a given value.
    ///
    /// ```no_run
    /// use futures::TryStreamExt;
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let key = "QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA";
    /// let res = client.dht_provide(key).try_collect::<Vec<_>>();
    /// ```
    ///
    fn dht_provide(&self, key: &str) -> BoxStream<response::DhtProvideResponse, Self::Error> {
        impl_stream_api_response! {
            (self, request::DhtProvide { key }, None) => request_stream_json
        }
    }

    /// Write a key/value pair to the DHT.
    ///
    /// ```no_run
    /// use futures::TryStreamExt;
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.dht_put("test", "Hello World!").try_collect::<Vec<_>>();
    /// ```
    ///
    fn dht_put(&self, key: &str, value: &str) -> BoxStream<response::DhtPutResponse, Self::Error> {
        impl_stream_api_response! {
            (self, request::DhtPut { key, value }, None) => request_stream_json
        }
    }

    /// Find the closest peer given the peer ID by querying the DHT.
    ///
    /// ```no_run
    /// use futures::TryStreamExt;
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let peer = "QmSoLPppuBtQSGwKDZT2M73ULpjvfd3aZ6ha4oFGL1KrGM";
    /// let res = client.dht_query(peer).try_collect::<Vec<_>>();
    /// ```
    ///
    fn dht_query(&self, peer: &str) -> BoxStream<response::DhtQueryResponse, Self::Error> {
        impl_stream_api_response! {
            (self, request::DhtQuery { peer }, None) => request_stream_json
        }
    }

    /// Clear inactive requests from the log.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.diag_cmds_clear();
    /// ```
    ///
    async fn diag_cmds_clear(&self) -> Result<response::DiagCmdsClearResponse, Self::Error> {
        self.request_empty(request::DiagCmdsClear, None).await
    }

    /// Set how long to keep inactive requests in the log.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.diag_cmds_set_time("1m");
    /// ```
    ///
    async fn diag_cmds_set_time(
        &self,
        time: &str,
    ) -> Result<response::DiagCmdsSetTimeResponse, Self::Error> {
        self.request_empty(request::DiagCmdsSetTime { time }, None)
            .await
    }

    /// Print system diagnostic information.
    ///
    /// Note: There isn't good documentation on what this call is supposed to return.
    /// It might be platform dependent, but if it isn't, this can be fixed to return
    /// an actual object.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.diag_sys();
    /// ```
    ///
    async fn diag_sys(&self) -> Result<response::DiagSysResponse, Self::Error> {
        self.request_string(request::DiagSys, None).await
    }

    /// Resolve DNS link.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.dns("ipfs.io", true);
    /// ```
    ///
    async fn dns(&self, link: &str, recursive: bool) -> Result<response::DnsResponse, Self::Error> {
        self.request(request::Dns { link, recursive }, None).await
    }

    /// List directory for Unix filesystem objects.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.file_ls("/ipns/ipfs.io");
    /// ```
    ///
    async fn file_ls(&self, path: &str) -> Result<response::FileLsResponse, Self::Error> {
        self.request(request::FileLs { path }, None).await
    }

    /// Copy files into MFS.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.files_cp("/path/to/file", "/dest");
    /// ```
    ///
    async fn files_cp(
        &self,
        path: &str,
        dest: &str,
    ) -> Result<response::FilesCpResponse, Self::Error> {
        self.files_cp_with_options(request::FilesCp {
            path,
            dest,
            ..Default::default()
        })
        .await
    }

    /// Copy files into MFS.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.files_cp("/path/to/file", "/dest");
    /// ```
    ///
    async fn files_cp_with_options(
        &self,
        options: request::FilesCp<'_>,
    ) -> Result<response::FilesCpResponse, Self::Error> {
        self.request_empty(options, None).await
    }

    /// Flush a path's data to disk.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.files_flush(None);
    /// let res = client.files_flush(Some("/tmp"));
    /// ```
    ///
    async fn files_flush(
        &self,
        path: Option<&str>,
    ) -> Result<response::FilesFlushResponse, Self::Error> {
        self.request_empty(request::FilesFlush { path }, None).await
    }

    /// List directories in MFS.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.files_ls(None);
    /// let res = client.files_ls(Some("/tmp"));
    /// ```
    ///
    async fn files_ls(&self, path: Option<&str>) -> Result<response::FilesLsResponse, Self::Error> {
        self.files_ls_with_options(request::FilesLs {
            path,
            ..Default::default()
        })
        .await
    }

    /// List directories in MFS..
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// #[cfg(feature = "with-builder")]
    /// let req = ipfs_api::request::FilesLs::builder()
    ///     // .path("/") // defaults to /
    ///     .unsorted(false)
    ///     .long(true)
    ///     .build();
    /// #[cfg(not(feature = "with-builder"))]
    /// let req = ipfs_api::request::FilesLs {
    ///     path: None, // defaults to /
    ///     unsorted: Some(false),
    ///     long: Some(true),
    /// };
    /// let res = client.files_ls_with_options(req);
    /// ```
    ///
    /// Defaults to `-U`, so the output is unsorted.
    ///
    async fn files_ls_with_options(
        &self,
        options: request::FilesLs<'_>,
    ) -> Result<response::FilesLsResponse, Self::Error> {
        self.request(options, None).await
    }

    /// Make directories in MFS.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.files_mkdir("/test", false);
    /// let res = client.files_mkdir("/test/nested/dir", true);
    /// ```
    ///
    async fn files_mkdir(
        &self,
        path: &str,
        parents: bool,
    ) -> Result<response::FilesMkdirResponse, Self::Error> {
        self.files_mkdir_with_options(request::FilesMkdir {
            path,
            parents: Some(parents),
            ..Default::default()
        })
        .await
    }

    /// Make directories in MFS.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// #[cfg(feature = "with-builder")]
    /// let req = ipfs_api::request::FilesMkdir::builder()
    ///     .path("/test/nested/dir")
    ///     .parents(true)
    ///     .flush(false)
    ///     .build();
    /// #[cfg(not(feature = "with-builder"))]
    /// let req = ipfs_api::request::FilesMkdir {
    ///     path: "/test/nested/dir",
    ///     parents: Some(true),
    ///     flush: Some(false),
    ///     .. Default::default()
    /// };
    /// let res = client.files_mkdir_with_options(req);
    /// ```
    ///
    async fn files_mkdir_with_options(
        &self,
        options: request::FilesMkdir<'_>,
    ) -> Result<response::FilesMkdirResponse, Self::Error> {
        self.request_empty(options, None).await
    }

    /// Copy files into MFS.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.files_mv("/test/tmp.json", "/test/file.json");
    /// ```
    ///
    async fn files_mv(
        &self,
        path: &str,
        dest: &str,
    ) -> Result<response::FilesMvResponse, Self::Error> {
        self.files_mv_with_options(request::FilesMv {
            path,
            dest,
            ..Default::default()
        })
        .await
    }

    /// Copy files into MFS.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.files_mv_with_options(
    ///     ipfs_api::request::FilesMv {
    ///         path: "/test/tmp.json",
    ///         dest: "/test/file.json",
    ///         flush: Some(false),
    ///     }
    /// );
    /// ```
    ///
    async fn files_mv_with_options(
        &self,
        options: request::FilesMv<'_>,
    ) -> Result<response::FilesMvResponse, Self::Error> {
        self.request_empty(options, None).await
    }

    /// Read a file in MFS.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.files_read("/test/file.json");
    /// ```
    ///
    fn files_read(&self, path: &str) -> BoxStream<Bytes, Self::Error> {
        self.files_read_with_options(request::FilesRead {
            path,
            ..request::FilesRead::default()
        })
    }

    /// Read a file in MFS.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// #[cfg(feature = "with-builder")]
    /// let req = ipfs_api::request::FilesRead::builder()
    ///     .path("/test/file.json")
    ///     .offset(1024)
    ///     .count(8)
    ///     .build();
    /// #[cfg(not(feature = "with-builder"))]
    /// let req = ipfs_api::request::FilesRead {
    ///     path: "/test/file.json",
    ///     offset: Some(1024),
    ///     count: Some(8),
    /// };
    /// let res = client.files_read_with_options(req);
    /// ```
    ///
    fn files_read_with_options(
        &self,
        options: request::FilesRead,
    ) -> BoxStream<Bytes, Self::Error> {
        impl_stream_api_response! { (self, options, None) => request_stream_bytes }
    }

    /// Remove a file in MFS.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.files_rm("/test/dir", true);
    /// let res = client.files_rm("/test/file.json", false);
    /// ```
    ///
    async fn files_rm(
        &self,
        path: &str,
        recursive: bool,
    ) -> Result<response::FilesRmResponse, Self::Error> {
        self.files_rm_with_options(request::FilesRm {
            path,
            recursive: Some(recursive),
            ..Default::default()
        })
        .await
    }

    /// Remove a file in MFS.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// #[cfg(feature = "with-builder")]
    /// let req = ipfs_api::request::FilesRm::builder()
    ///     .path("/test/somefile.json")
    ///     .recursive(false)
    ///     .flush(false)
    ///     .build();
    /// #[cfg(not(feature = "with-builder"))]
    /// let req = ipfs_api::request::FilesRm {
    ///     path: "/test/somefile.json",
    ///     recursive: Some(false),
    ///     flush: Some(false),
    /// };
    /// let res = client.files_rm_with_options(req);
    /// ```
    ///
    async fn files_rm_with_options(
        &self,
        options: request::FilesRm<'_>,
    ) -> Result<response::FilesRmResponse, Self::Error> {
        self.request_empty(options, None).await
    }

    /// Display a file's status in MFS.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.files_stat("/test/file.json");
    /// ```
    ///
    async fn files_stat(&self, path: &str) -> Result<response::FilesStatResponse, Self::Error> {
        self.files_stat_with_options(request::FilesStat {
            path,
            ..Default::default()
        })
        .await
    }

    /// Display a file's status in MFS.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.files_stat_with_options(
    ///     ipfs_api::request::FilesStat {
    ///         path: "/test/dir/",
    ///         with_local: Some(true),
    ///     }
    /// );
    /// ```
    ///
    async fn files_stat_with_options(
        &self,
        options: request::FilesStat<'_>,
    ) -> Result<response::FilesStatResponse, Self::Error> {
        self.request(options, None).await
    }

    /// Write to a mutable file in the filesystem.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    /// use std::fs::File;
    ///
    /// let client = IpfsClient::default();
    /// let file = File::open("test.json").unwrap();
    /// let res = client.files_write("/test/file.json", true, true, file);
    /// ```
    ///
    async fn files_write<R>(
        &self,
        path: &str,
        create: bool,
        truncate: bool,
        data: R,
    ) -> Result<response::FilesWriteResponse, Self::Error>
    where
        R: 'static + Read + Send + Sync + Unpin,
    {
        let options = request::FilesWrite {
            path,
            create: Some(create),
            truncate: Some(truncate),
            ..request::FilesWrite::default()
        };

        self.files_write_with_options(options, data).await
    }

    /// Write to a mutable file in the filesystem.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    /// use std::io::Cursor;
    ///
    /// let client = IpfsClient::default();
    /// let data = Cursor::new((1..128).collect::<Vec<u8>>());
    /// #[cfg(feature = "with-builder")]
    /// let req = ipfs_api::request::FilesWrite::builder()
    ///     .path("/test/outfile.bin")
    ///     .create(false)
    ///     .truncate(false)
    ///     .offset(1 << 20)
    ///     .flush(false)
    ///     // see FilesWriteBuilder for the full set of options
    ///     .build();
    /// #[cfg(not(feature = "with-builder"))]
    /// let req = ipfs_api::request::FilesWrite {
    ///     path: "/test/outfile.bin",
    ///     create: Some(false),
    ///     truncate: Some(false),
    ///     offset: Some(1 << 20),
    ///     flush: Some(false),
    ///     .. Default::default()
    /// };
    /// let res = client.files_write_with_options(req, data);
    /// ```
    ///
    async fn files_write_with_options<R>(
        &self,
        options: request::FilesWrite<'_>,
        data: R,
    ) -> Result<response::FilesWriteResponse, Self::Error>
    where
        R: 'static + Read + Send + Sync + Unpin,
    {
        let mut form = multipart::Form::default();

        form.add_reader("data", data);

        self.request_empty(options, Some(form)).await
    }

    /// Change the cid version or hash function of the root node of a given path.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    /// use std::fs::File;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.files_chcid("/test/", 1);
    /// ```
    ///
    /// Not specifying a byte `count` writes the entire input.
    ///
    async fn files_chcid(
        &self,
        path: &str,
        cid_version: i32,
    ) -> Result<response::FilesChcidResponse, Self::Error> {
        self.request_empty(
            request::FilesChcid {
                path: Some(path),
                cid_version: Some(cid_version),
                ..Default::default()
            },
            None,
        )
        .await
    }

    /// Change the cid version or hash function of the root node of a given path.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    /// use std::fs::File;
    ///
    /// let client = IpfsClient::default();
    /// #[cfg(feature = "with-builder")]
    /// let req = ipfs_api::request::FilesChcid::builder()
    ///     .path("/test/")
    ///     .cid_version(1)
    ///     .hash("sha3-512")
    ///     .flush(true)
    ///     .build();
    /// #[cfg(not(feature = "with-builder"))]
    /// let req = ipfs_api::request::FilesChcid {
    ///     path: Some("/test/"),
    ///     cid_version: Some(1),
    ///     hash: Some("sha3-512"),
    ///     flush: Some(false),
    /// };
    /// let res = client.files_chcid_with_options(req);
    /// ```
    ///
    /// Not specifying a byte `count` writes the entire input.
    ///
    async fn files_chcid_with_options(
        &self,
        options: request::FilesChcid<'_>,
    ) -> Result<response::FilesChcidResponse, Self::Error> {
        self.request_empty(options, None).await
    }

    /// List blocks that are both in the filestore and standard block storage.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.filestore_dups();
    /// ```
    ///
    fn filestore_dups(&self) -> BoxStream<response::FilestoreDupsResponse, Self::Error> {
        impl_stream_api_response! {
            (self, request::FilestoreDups, None) => request_stream_json
        }
    }

    /// List objects in filestore.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.filestore_ls(
    ///     Some("QmYPP3BovR2m8UqCZxFbdXSit6SKgExxDkFAPLqiGsap4X")
    /// );
    /// ```
    ///
    fn filestore_ls(
        &self,
        cid: Option<&str>,
    ) -> BoxStream<response::FilestoreLsResponse, Self::Error> {
        impl_stream_api_response! {
            (self, request::FilestoreLs { cid }, None) => request_stream_json
        }
    }

    /// Verify objects in filestore.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.filestore_verify(None);
    /// ```
    ///
    fn filestore_verify(
        &self,
        cid: Option<&str>,
    ) -> BoxStream<response::FilestoreVerifyResponse, Self::Error> {
        impl_stream_api_response! {
            (self, request::FilestoreVerify{ cid }, None) => request_stream_json
        }
    }

    /// Download Ipfs object.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.get("/test/file.json");
    /// ```
    ///
    fn get(&self, path: &str) -> BoxStream<Bytes, Self::Error> {
        impl_stream_api_response! {
            (self, request::Get { path }, None) => request_stream_bytes
        }
    }

    /// Returns information about a peer.
    ///
    /// If `peer` is `None`, returns information about you.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.id(None);
    /// let res = client.id(Some("QmSoLPppuBtQSGwKDZT2M73ULpjvfd3aZ6ha4oFGL1KrGM"));
    /// ```
    ///
    async fn id(&self, peer: Option<&str>) -> Result<response::IdResponse, Self::Error> {
        self.request(request::Id { peer }, None).await
    }

    /// Create a new keypair.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient, KeyType};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.key_gen("test", KeyType::Rsa, 64);
    /// ```
    ///
    async fn key_gen(
        &self,
        name: &str,
        kind: request::KeyType,
        size: i32,
    ) -> Result<response::KeyGenResponse, Self::Error> {
        self.request(request::KeyGen { name, kind, size }, None)
            .await
    }

    /// List all local keypairs.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.key_list();
    /// ```
    ///
    async fn key_list(&self) -> Result<response::KeyListResponse, Self::Error> {
        self.request(request::KeyList, None).await
    }

    /// Rename a keypair.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.key_rename("key_0", "new_name", false);
    /// ```
    ///
    async fn key_rename(
        &self,
        name: &str,
        new: &str,
        force: bool,
    ) -> Result<response::KeyRenameResponse, Self::Error> {
        self.request(request::KeyRename { name, new, force }, None)
            .await
    }

    /// Remove a keypair.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.key_rm("key_0");
    /// ```
    ///
    async fn key_rm(&self, name: &str) -> Result<response::KeyRmResponse, Self::Error> {
        self.request(request::KeyRm { name }, None).await
    }

    /// Change the logging level for a logger.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient, Logger, LoggingLevel};
    /// use std::borrow::Cow;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.log_level(Logger::All, LoggingLevel::Debug);
    /// let res = client.log_level(
    ///     Logger::Specific(Cow::Borrowed("web")),
    ///     LoggingLevel::Warning
    /// );
    /// ```
    ///
    async fn log_level(
        &self,
        logger: request::Logger<'_>,
        level: request::LoggingLevel,
    ) -> Result<response::LogLevelResponse, Self::Error> {
        self.request(request::LogLevel { logger, level }, None)
            .await
    }

    /// List all logging subsystems.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.log_ls();
    /// ```
    ///
    async fn log_ls(&self) -> Result<response::LogLsResponse, Self::Error> {
        self.request(request::LogLs, None).await
    }

    /// Read the event log.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.log_tail();
    /// ```
    ///
    fn log_tail(&self) -> BoxStream<String, Self::Error> {
        impl_stream_api_response! {
            (self, request::LogTail, None) |req| => {
                self.request_stream(req, |res| {
                    Box::new(Self::process_stream_response(res, LineDecoder).map_err(Self::Error::from))
                })
            }
        }
    }

    /// List the contents of an Ipfs multihash.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.ls("/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY");
    /// ```
    ///
    async fn ls(&self, path: &str) -> Result<response::LsResponse, Self::Error> {
        self.request(
            request::Ls {
                path,
                ..Default::default()
            },
            None,
        )
        .await
    }

    /// List the contents of an Ipfs multihash.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// #[cfg(feature = "with-builder")]
    /// let _ = client.ls_with_options(ipfs_api::request::Ls::builder()
    ///     .path("/ipfs/QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zR1n")
    ///     .build()
    /// );
    /// #[cfg(not(feature = "with-builder"))]
    /// let _ = client.ls_with_options(ipfs_api::request::Ls {
    ///     path: "/ipfs/QmdfTbBqBPQ7VNxZEYEj14VmRuZBkqFbiwReogJgS1zR1n",
    ///     // Example options for fast listing
    ///     stream: Some(true),
    ///     resolve_type: Some(false),
    ///     size: Some(false),
    /// });
    /// ```
    ///
    fn ls_with_options(
        &self,
        options: request::Ls<'_>,
    ) -> BoxStream<response::LsResponse, Self::Error> {
        impl_stream_api_response! {
            (self, options, None) => request_stream_json
        }
    }

    // TODO /mount

    /// Publish an IPFS path to IPNS.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.name_publish(
    ///     "/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY",
    ///     false,
    ///     Some("12h"),
    ///     None,
    ///     None
    /// );
    /// ```
    ///
    async fn name_publish(
        &self,
        path: &str,
        resolve: bool,
        lifetime: Option<&str>,
        ttl: Option<&str>,
        key: Option<&str>,
    ) -> Result<response::NamePublishResponse, Self::Error> {
        self.request(
            request::NamePublish {
                path,
                resolve,
                lifetime,
                ttl,
                key,
            },
            None,
        )
        .await
    }

    /// Resolve an IPNS name.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.name_resolve(
    ///     Some("/ipns/ipfs.io"),
    ///     true,
    ///     false
    /// );
    /// ```
    ///
    async fn name_resolve(
        &self,
        name: Option<&str>,
        recursive: bool,
        nocache: bool,
    ) -> Result<response::NameResolveResponse, Self::Error> {
        self.request(
            request::NameResolve {
                name,
                recursive,
                nocache,
            },
            None,
        )
        .await
    }

    /// Output the raw bytes of an Ipfs object.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.object_data("/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY");
    /// ```
    ///
    fn object_data(&self, key: &str) -> BoxStream<Bytes, Self::Error> {
        impl_stream_api_response! {
            (self, request::ObjectData { key }, None) => request_stream_bytes
        }
    }

    /// Returns the diff of two Ipfs objects.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.object_diff(
    ///     "/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY",
    ///     "/ipfs/QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA"
    /// );
    /// ```
    ///
    async fn object_diff(
        &self,
        key0: &str,
        key1: &str,
    ) -> Result<response::ObjectDiffResponse, Self::Error> {
        self.request(request::ObjectDiff { key0, key1 }, None).await
    }

    /// Returns the data in an object.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.object_get("/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY");
    /// ```
    ///
    async fn object_get(&self, key: &str) -> Result<response::ObjectGetResponse, Self::Error> {
        self.request(request::ObjectGet { key }, None).await
    }

    /// Returns the links that an object points to.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.object_links("/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY");
    /// ```
    ///
    async fn object_links(&self, key: &str) -> Result<response::ObjectLinksResponse, Self::Error> {
        self.request(request::ObjectLinks { key }, None).await
    }

    /// Create a new object.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient, ObjectTemplate};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.object_new(None);
    /// let res = client.object_new(Some(ObjectTemplate::UnixFsDir));
    /// ```
    ///
    async fn object_new(
        &self,
        template: Option<request::ObjectTemplate>,
    ) -> Result<response::ObjectNewResponse, Self::Error> {
        self.request(request::ObjectNew { template }, None).await
    }

    /// Add a directory link to an object.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.object_patch_add_link(
    ///     "QmUNLLsPACCz1vLxQVkXqqLX5R1X345qqfHbsf67hvA3Nn",
    ///     "hello_world.txt",
    ///     "QmfM2r8seH2GiRaC4esTjeraXEachRt8ZsSeGaWTPLyMoG",
    ///     false
    /// );
    /// let res = client.object_patch_add_link(
    ///     "QmcXu68EVrtSEQ8SoPCWAfKJ9JqY6jnZyyiizRwksnt3kv",
    ///     "hello/dad.txt",
    ///     "Qma1UVHKYFkk6cGo3V1VmyMxRb1Bpd9SBbXdZURk28VtND",
    ///     true
    /// );
    /// ```
    ///
    async fn object_patch_add_link(
        &self,
        folder: &str,
        name: &str,
        key: &str,
        create: bool,
    ) -> Result<response::ObjectPatchAddLinkResponse, Self::Error> {
        self.request(
            request::ObjectPatchAddLink {
                folder,
                name,
                key,
                create,
            },
            None,
        )
        .await
    }

    // TODO /object/patch/append-data

    // TODO /object/patch/rm-link

    // TODO /object/patch/set-data

    // TODO /object/put

    /// Returns the stats for an object.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.object_stat("/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY");
    /// ```
    ///
    async fn object_stat(&self, key: &str) -> Result<response::ObjectStatResponse, Self::Error> {
        self.request(request::ObjectStat { key }, None).await
    }

    // TODO /p2p/listener/close

    // TODO /p2p/listener/ls

    // TODO /p2p/listener/open

    // TODO /p2p/stream/close

    // TODO /p2p/stream/dial

    // TODO /p2p/stream/ls

    /// Pins a new object.
    ///
    /// The "recursive" option tells the server whether to
    /// pin just the top-level object, or all sub-objects
    /// it depends on.  For most cases you want it to be `true`.
    ///
    /// Does not yet implement the "progress" agument because
    /// reading it is kinda squirrelly.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.pin_add("QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ", true);
    /// ```
    ///
    async fn pin_add(
        &self,
        key: &str,
        recursive: bool,
    ) -> Result<response::PinAddResponse, Self::Error> {
        self.request(
            request::PinAdd {
                key,
                recursive: Some(recursive),
                progress: false,
            },
            None,
        )
        .await
    }

    /// Returns a list of pinned objects in local storage.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.pin_ls(None, None);
    /// let res = client.pin_ls(
    ///     Some("/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY"),
    ///     None
    /// );
    /// let res = client.pin_ls(None, Some("direct"));
    /// ```
    ///
    async fn pin_ls(
        &self,
        key: Option<&str>,
        typ: Option<&str>,
    ) -> Result<response::PinLsResponse, Self::Error> {
        self.request(request::PinLs { key, typ }, None).await
    }

    /// Removes a pinned object from local storage.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.pin_rm(
    ///     "/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY",
    ///     false
    /// );
    /// let res = client.pin_rm(
    ///     "/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY",
    ///     true
    /// );
    /// ```
    ///
    async fn pin_rm(
        &self,
        key: &str,
        recursive: bool,
    ) -> Result<response::PinRmResponse, Self::Error> {
        self.request(request::PinRm { key, recursive }, None).await
    }

    // TODO /pin/update

    // TODO /pin/verify

    /// Pings a peer.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.ping("QmSoLV4Bbm51jM9C4gDYZQ9Cy3U6aXMJDAbzgu2fzaDs64", None);
    /// let res = client.ping("QmSoLV4Bbm51jM9C4gDYZQ9Cy3U6aXMJDAbzgu2fzaDs64", Some(15));
    /// ```
    ///
    fn ping(
        &self,
        peer: &str,
        count: Option<i32>,
    ) -> BoxStream<response::PingResponse, Self::Error> {
        impl_stream_api_response! {
            (self, request::Ping { peer, count }, None) => request_stream_json
        }
    }

    /// List subscribed pubsub topics.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.pubsub_ls();
    /// ```
    ///
    async fn pubsub_ls(&self) -> Result<response::PubsubLsResponse, Self::Error> {
        self.request(request::PubsubLs, None).await
    }

    /// List peers that are being published to.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.pubsub_peers(Option::<String>::None);
    /// let res = client.pubsub_peers(Some("feed"));
    /// ```
    ///
    async fn pubsub_peers<T>(
        &self,
        topic: Option<T>,
    ) -> Result<response::PubsubPeersResponse, Self::Error>
    where
        T: AsRef<[u8]> + Send + Sync,
    {
        match topic {
            Some(topic) => {
                self.request(
                    request::PubsubPeers {
                        topic: Some(topic.as_ref()),
                    },
                    None,
                )
                .await
            }
            None => {
                self.request(request::PubsubPeers { topic: None }, None)
                    .await
            }
        }
    }

    /// Publish a message to a topic.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    /// use std::io::Cursor;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.pubsub_pub("feed", Cursor::new("Hello World!"));
    /// ```
    ///
    async fn pubsub_pub<T, R>(
        &self,
        topic: T,
        data: R,
    ) -> Result<response::PubsubPubResponse, Self::Error>
    where
        T: AsRef<[u8]> + Send + Sync,
        R: 'static + Read + Send + Sync + Unpin,
    {
        let mut form = multipart::Form::default();

        form.add_reader("data", data);

        self.request_empty(
            request::PubsubPub {
                topic: topic.as_ref(),
            },
            Some(form),
        )
        .await
    }

    /// Subscribes to a pubsub topic.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.pubsub_sub("feed");
    /// let res = client.pubsub_sub("feed");
    /// ```
    ///
    fn pubsub_sub<T>(&self, topic: T) -> BoxStream<response::PubsubSubResponse, Self::Error>
    where
        T: AsRef<[u8]>,
    {
        impl_stream_api_response! {
            (self, request::PubsubSub { topic: topic.as_ref() }, None) => request_stream_json
        }
    }

    /// Gets a list of local references.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.refs_local();
    /// ```
    ///
    fn refs_local(&self) -> BoxStream<response::RefsLocalResponse, Self::Error> {
        impl_stream_api_response! {
            (self, request::RefsLocal, None) => request_stream_json
        }
    }

    // TODO /repo/fsck

    // TODO /repo/gc

    // TODO /repo/stat

    // TODO /repo/verify

    // TODO /repo/version

    // TODO /resolve

    /// Shutdown the Ipfs daemon.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.shutdown();
    /// ```
    ///
    async fn shutdown(&self) -> Result<response::ShutdownResponse, Self::Error> {
        self.request_empty(request::Shutdown, None).await
    }

    /// Returns bitswap stats.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.stats_bitswap();
    /// ```
    ///
    async fn stats_bitswap(&self) -> Result<response::StatsBitswapResponse, Self::Error> {
        self.request(request::StatsBitswap, None).await
    }

    /// Returns bandwidth stats.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.stats_bw();
    /// ```
    ///
    async fn stats_bw(&self) -> Result<response::StatsBwResponse, Self::Error> {
        self.request(request::StatsBw, None).await
    }

    /// Returns repo stats.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.stats_repo();
    /// ```
    ///
    async fn stats_repo(&self) -> Result<response::StatsRepoResponse, Self::Error> {
        self.request(request::StatsRepo, None).await
    }

    // TODO /swarm/addrs/listen

    /// Return a list of local addresses.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.swarm_addrs_local();
    /// ```
    ///
    async fn swarm_addrs_local(&self) -> Result<response::SwarmAddrsLocalResponse, Self::Error> {
        self.request(request::SwarmAddrsLocal, None).await
    }

    /// Connect to a given peer
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.swarm_connect("/dns4/production-ipfs-cluster-us-east-1-node2.runfission.com/tcp/4003/wss/p2p/12D3KooWQ2hL9NschcJ1Suqa1TybJc2ZaacqoQMBT3ziFC7Ye2BZ");
    /// ```
    ///
    async fn swarm_connect(
        &self,
        peer: &str,
    ) -> Result<response::SwarmConnectResponse, Self::Error> {
        self.request(request::SwarmConnect { peer }, None).await
    }

    // TODO /swarm/disconnect

    // TODO /swarm/filters/add

    // TODO /swarm/filters/rm

    /// Return a list of peers with open connections.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.swarm_peers();
    /// ```
    ///
    async fn swarm_peers(&self) -> Result<response::SwarmPeersResponse, Self::Error> {
        self.request(request::SwarmPeers, None).await
    }

    /// Add a tar file to Ipfs.
    ///
    /// Note: `data` should already be a tar file. If it isn't the Api will return
    /// an error.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    /// use std::fs::File;
    ///
    /// let client = IpfsClient::default();
    /// let tar = File::open("/path/to/file.tar").unwrap();
    /// let res = client.tar_add(tar);
    /// ```
    ///
    async fn tar_add<R>(&self, data: R) -> Result<response::TarAddResponse, Self::Error>
    where
        R: 'static + Read + Send + Sync + Unpin,
    {
        let mut form = multipart::Form::default();

        form.add_reader("file", data);

        self.request(request::TarAdd, Some(form)).await
    }

    /// Export a tar file from Ipfs.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.tar_cat("/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY");
    /// ```
    ///
    fn tar_cat(&self, path: &str) -> BoxStream<Bytes, Self::Error> {
        impl_stream_api_response! {
            (self, request::TarCat { path }, None) => request_stream_bytes
        }
    }

    /// Returns information about the Ipfs server version.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsApi, IpfsClient};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.version();
    /// ```
    ///
    async fn version(&self) -> Result<response::VersionResponse, Self::Error> {
        self.request(request::Version, None).await
    }
}

impl<B> IpfsApi for B where B: Backend {}
