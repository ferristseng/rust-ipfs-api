// Copyright 2021 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::{request, response, Backend};
use async_trait::async_trait;
use bytes::Bytes;
use futures::{future, FutureExt, Stream, StreamExt, TryStreamExt};
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
        match $self.build_base_request(&$req, $form) {
            Ok($var) => Box::new($impl),
            Err(e) => Box::new(future::err(e).into_stream()),
        }
    };
}

#[async_trait(?Send)]
pub trait IpfsApi: Backend {
    /// Add file to Ipfs.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    /// use std::io::Cursor;
    ///
    /// let client = IpfsClient::default();
    /// let data = Cursor::new("Hello World!");
    /// let res = client.add(data);
    /// ```
    ///
    async fn add<R>(&self, data: R) -> Result<response::AddResponse, Self::Error>
    where
        R: 'static + Read + Send + Sync,
    {
        self.add_with_options(data, request::Add::default()).await
    }

    /// Add a file to IPFS with options.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
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
        R: 'static + Read + Send + Sync,
    {
        let mut form = Self::MultipartForm::default();

        //form.add_reader("path", data);

        self.request(add, Some(form)).await
    }

    /// Add a path to Ipfs. Can be a file or directory.
    /// A hard limit of 128 open file descriptors is set such
    /// that any small additional files are stored in-memory.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let path = "./src";
    /// let res = client.add_path(path);
    /// ```
    ///
    async fn add_path<P>(&self, path: P) -> Result<Vec<response::AddResponse>, Self::Error>
    where
        P: AsRef<Path>,
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
        let mut form = Self::MultipartForm::default();

        for (path, file_size) in paths_to_add {
            let mut file = File::open(&path).map_err(|e| crate::Error::Io(e))?;
            let file_name = match prefix {
                Some(prefix) => path.strip_prefix(prefix).unwrap(),
                None => path.as_path(),
            }
            .to_string_lossy();

            if it < FILE_DESCRIPTOR_LIMIT {
                //form.add_reader_file("path", file, file_name);

                it += 1;
            } else {
                let mut buf = Vec::with_capacity(file_size as usize);
                let _ = file
                    .read_to_end(&mut buf)
                    .map_err(|e| crate::Error::Io(e))?;

                //form.add_reader_file("path", Cursor::new(buf), file_name);
            }
        }

        let req = self.build_base_request(&request::Add::default(), Some(form))?;

        self.request_stream_json(req).try_collect().await
    }

    /// Returns the current ledger for a peer.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let hash = "QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA";
    /// let res = client
    ///     .block_get(hash)
    ///     .map_ok(|chunk| chunk.to_vec())
    ///     .try_concat();
    /// ```
    ///
    fn block_get(&self, hash: &str) -> Box<dyn Stream<Item = Result<Bytes, Self::Error>> + Unpin> {
        impl_stream_api_response! {
            (self, request::BlockGet { hash }, None) => request_stream_bytes
        }
    }

    /// Store input as an IPFS block.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    /// use std::io::Cursor;
    ///
    /// let client = IpfsClient::default();
    /// let data = Cursor::new("Hello World!");
    /// let res = client.block_put(data);
    /// ```
    ///
    async fn block_put<R>(&self, data: R) -> Result<response::BlockPutResponse, Self::Error>
    where
        R: 'static + Read + Send + Sync,
    {
        let mut form = Self::MultipartForm::default();

        //form.add_reader("data", data);

        self.request(request::BlockPut, Some(form)).await
    }

    /// Removes an IPFS block.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let hash = "QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA";
    /// let res = client
    ///     .cat(hash)
    ///     .map_ok(|chunk| chunk.to_vec())
    ///     .try_concat();
    /// ```
    ///
    fn cat(&self, path: &str) -> Box<dyn Stream<Item = Result<Bytes, Self::Error>>> {
        impl_stream_api_response! {
            (self, request::Cat { path }, None) => request_stream_bytes
        }
    }

    /// List available commands that the server accepts.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
    /// use std::io::Cursor;
    ///
    /// let client = IpfsClient::default();
    /// let config = Cursor::new("{..json..}");
    /// let res = client.config_replace(config);
    /// ```
    ///
    async fn config_replace<R>(&self, data: R) -> Result<response::ConfigReplaceResponse, Self::Error>
    where
        R: 'static + Read + Send + Sync,
    {
        let mut form = Self::MultipartForm::default();

        //form.add_reader("file", data);

        self.request_empty(request::ConfigReplace, Some(form)).await
    }

    /// Show the current config of the server.
    ///
    /// Returns an unparsed json string, due to an unclear spec.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let hash = "QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA";
    /// let res = client
    ///     .dag_get(hash)
    ///     .map_ok(|chunk| chunk.to_vec())
    ///     .try_concat();
    /// ```
    ///
    fn dag_get(&self, path: &str) -> Box<dyn Stream<Item = Result<Bytes, Self::Error>>> {
        impl_stream_api_response! {
            (self, request::DagGet { path }, None) => request_stream_bytes
        }
    }

    /// Add a DAG node to Ipfs.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    /// use std::io::Cursor;
    ///
    /// let client = IpfsClient::default();
    /// let data = Cursor::new(r#"{ "hello" : "world" }"#);
    /// let res = client.dag_put(data);
    /// ```
    ///
    async fn dag_put<R>(&self, data: R) -> Result<response::DagPutResponse, Self::Error>
    where
        R: 'static + Read + Send + Sync,
    {
        let mut form = Self::MultipartForm::default();

        //form.add_reader("object data", data);

        self.request(request::DagPut, Some(form)).await
    }

    // TODO /dag/resolve

    /// Query the DHT for all of the multiaddresses associated with a Peer ID.
    ///
    /// ```no_run
    /// use futures::TryStreamExt;
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let peer = "QmSoLPppuBtQSGwKDZT2M73ULpjvfd3aZ6ha4oFGL1KrGM";
    /// let res = client.dht_findpeer(peer).try_collect::<Vec<_>>();
    /// ```
    ///
    fn dht_findpeer(
        &self,
        peer: &str,
    ) -> Box<dyn Stream<Item = Result<response::DhtFindPeerResponse, Self::Error>> + Unpin> {
        impl_stream_api_response! {
            (self, request::DhtFindPeer { peer }, None) => request_stream_json
        }
    }

    /// Find peers in the DHT that can provide a specific value given a key.
    ///
    /// ```no_run
    /// use futures::TryStreamExt;
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let key = "QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA";
    /// let res = client.dht_findprovs(key).try_collect::<Vec<_>>();
    /// ```
    ///
    fn dht_findprovs(
        &self,
        key: &str,
    ) -> Box<dyn Stream<Item = Result<response::DhtFindProvsResponse, Self::Error>> + Unpin> {
        impl_stream_api_response! {
            (self, request::DhtFindProvs { key }, None) => request_stream_json
        }
    }

    /// Query the DHT for a given key.
    ///
    /// ```no_run
    /// use futures::TryStreamExt;
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let key = "QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA";
    /// let res = client.dht_get(key).try_collect::<Vec<_>>();
    /// ```
    ///
    fn dht_get(
        &self,
        key: &str,
    ) -> Box<dyn Stream<Item = Result<response::DhtGetResponse, Self::Error>>> {
        impl_stream_api_response! {
            (self, request::DhtGet { key }, None) => request_stream_json
        }
    }

    /// Announce to the network that you are providing a given value.
    ///
    /// ```no_run
    /// use futures::TryStreamExt;
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let key = "QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA";
    /// let res = client.dht_provide(key).try_collect::<Vec<_>>();
    /// ```
    ///
    fn dht_provide(
        &self,
        key: &str,
    ) -> Box<dyn Stream<Item = Result<response::DhtProvideResponse, Self::Error>>> {
        impl_stream_api_response! {
            (self, request::DhtProvide { key }, None) => request_stream_json
        }
    }

    /// Write a key/value pair to the DHT.
    ///
    /// ```no_run
    /// use futures::TryStreamExt;
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.dht_put("test", "Hello World!").try_collect::<Vec<_>>();
    /// ```
    ///
    fn dht_put(
        &self,
        key: &str,
        value: &str,
    ) -> Box<dyn Stream<Item = Result<response::DhtPutResponse, Self::Error>>> {
        impl_stream_api_response! {
            (self, request::DhtPut { key, value }, None) => request_stream_json
        }
    }

    /// Find the closest peer given the peer ID by querying the DHT.
    ///
    /// ```no_run
    /// use futures::TryStreamExt;
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let peer = "QmSoLPppuBtQSGwKDZT2M73ULpjvfd3aZ6ha4oFGL1KrGM";
    /// let res = client.dht_query(peer).try_collect::<Vec<_>>();
    /// ```
    ///
    fn dht_query(
        &self,
        peer: &str,
    ) -> Box<dyn Stream<Item = Result<response::DhtQueryResponse, Self::Error>>> {
        impl_stream_api_response! {
            (self, request::DhtQuery { peer }, None) => request_stream_json
        }
    }

    /// Clear inactive requests from the log.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
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
    /// use ipfs_api::IpfsClient;
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
}
