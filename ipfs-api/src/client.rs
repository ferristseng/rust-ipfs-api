// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use futures::{stream, Stream};
use futures::future::{Future, IntoFuture};
use header::Trailer;
use read::{JsonLineDecoder, LineDecoder, StreamReader};
use request::{self, ApiRequest};
use response::{self, Error, ErrorKind};
use hyper::{self, Body, Chunk, Request, Response, Uri, Method, StatusCode};
use hyper::client::{Client, HttpConnector};
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::Read;
use tokio_core::reactor::Handle;
use tokio_io::codec::{Decoder, FramedRead};


/// A future response returned by the reqwest HTTP client.
///
type AsyncResponse<T> = Box<Future<Item = T, Error = Error>>;


/// A future that returns a stream of responses.
///
type AsyncStreamResponse<T> = Box<Stream<Item = T, Error = Error>>;


/// Asynchronous Ipfs client.
///
pub struct IpfsClient {
    base: Uri,
    client: Client<HttpConnector, Body>,
}

impl IpfsClient {
    /// Creates a new `IpfsClient`.
    ///
    #[inline]
    pub fn new(
        handle: &Handle,
        host: &str,
        port: u16,
    ) -> Result<IpfsClient, hyper::error::UriError> {
        let base_path = IpfsClient::build_base_path(host, port)?;

        Ok(IpfsClient {
            base: base_path,
            client: Client::new(handle),
        })
    }

    /// Creates an `IpfsClient` connected to `localhost:5001`.
    ///
    pub fn default(handle: &Handle) -> IpfsClient {
        IpfsClient::new(handle, "localhost", 5001).unwrap()
    }

    /// Builds the base url path for the Ipfs api.
    ///
    fn build_base_path(host: &str, port: u16) -> Result<Uri, hyper::error::UriError> {
        format!("http://{}:{}/api/v0", host, port).parse()
    }

    /// Builds the url for an api call.
    ///
    fn build_base_request<Req>(&self, req: &Req) -> Result<Request, Error>
    where
        Req: ApiRequest + Serialize,
    {
        let url = format!(
            "{}{}?{}",
            self.base,
            Req::path(),
            ::serde_urlencoded::to_string(req)?
        );

        url.parse::<Uri>()
            .map(|url| Request::new(Method::Get, url))
            .map_err(From::from)
    }

    /// Builds an Api error from a response body.
    ///
    #[inline]
    fn build_error_from_body(chunk: Chunk) -> Error {
        match serde_json::from_slice(&chunk) {
            Ok(e) => ErrorKind::Api(e).into(),
            Err(_) => {
                match String::from_utf8(chunk.to_vec()) {
                    Ok(s) => ErrorKind::Uncategorized(s).into(),
                    Err(e) => e.into(),
                }
            }
        }
    }

    /// Processes a response that expects a json encoded body, returning an
    /// error or a deserialized json response.
    ///
    fn process_json_response<Res>(status: StatusCode, chunk: Chunk) -> Result<Res, Error>
    where
        for<'de> Res: 'static + Deserialize<'de>,
    {
        match status {
            StatusCode::Ok => serde_json::from_slice(&chunk).map_err(From::from),
            _ => Err(Self::build_error_from_body(chunk)),
        }
    }

    /// Processes a response that returns a stream of json deserializable
    /// results.
    ///
    fn process_stream_response<D, Res>(
        res: Response,
        decoder: D,
    ) -> Box<Stream<Item = Res, Error = Error>>
    where
        D: 'static + Decoder<Item = Res, Error = Error>,
        Res: 'static,
    {
        let err: Option<Error> = if let Some(trailer) = res.headers().get() {
            // Response has the Trailer header set, which is used
            // by Ipfs to indicate an error when preparing an output
            // stream.
            //
            match trailer {
                &Trailer::StreamError => Some(ErrorKind::StreamError.into()),
            }
        } else {
            None
        };

        let stream = FramedRead::new(StreamReader::new(res.body().from_err()), decoder);

        if let Some(inner) = err {
            // If there was an error while streaming data back, read
            // as much as possible from the stream, then return an error.
            //
            Box::new(stream.chain(stream::once(Err(inner))))
        } else {
            Box::new(stream)
        }
    }

    /// Sends a request and returns the raw response.
    ///
    /// Methods prefixed with `send_` work on a raw reqwest `RequestBuilder`
    /// instance.
    ///
    fn send_request(&self, req: Request) -> AsyncResponse<(StatusCode, Chunk)> {
        let res = self.client
            .request(req)
            .and_then(|res| {
                let status = res.status();

                res.body().concat2().map(move |chunk| (status, chunk))
            })
            .from_err();

        Box::new(res)
    }

    /// Sends a request and deserializes the response into Json.
    ///
    /// Methods prefixed with `send_` work on a raw reqwest `RequestBuilder`
    /// instance.
    ///
    fn send_request_json<Res>(&self, req: Request) -> AsyncResponse<Res>
    where
        for<'de> Res: 'static + Deserialize<'de>,
    {
        let res = self.send_request(req).into_future().and_then(
            |(status, chunk)| {
                IpfsClient::process_json_response(status, chunk)
            },
        );

        Box::new(res)
    }

    /// Generates a request, and returns the unprocessed response future.
    ///
    fn request_raw<Req>(&self, req: &Req) -> AsyncResponse<(StatusCode, Chunk)>
    where
        Req: ApiRequest + Serialize,
    {
        let res = self.build_base_request(req)
            .map(|req| self.send_request(req))
            .into_future()
            .flatten();

        Box::new(res)
    }

    /// Generic method for making a request to the Ipfs server, and getting
    /// a deserializable response.
    ///
    fn request<Req, Res>(&self, req: &Req) -> AsyncResponse<Res>
    where
        Req: ApiRequest + Serialize,
        for<'de> Res: 'static + Deserialize<'de>,
    {
        let res = self.build_base_request(req)
            .map(|req| self.send_request_json(req))
            .into_future()
            .flatten();

        Box::new(res)
    }

    /// Generic method for making a request to the Ipfs server, and getting
    /// a deserializable response.
    ///
    fn request_with_body<Req, Res, R>(&self, data: R, req: &Req) -> AsyncResponse<Res>
    where
        Req: ApiRequest + Serialize,
        for<'de> Res: 'static + Deserialize<'de>,
        R: 'static + Read + Send,
    {
        let res = self.build_base_request(req)
            .map(|req| self.send_request_json(req))
            .into_future()
            .flatten();

        Box::new(res)
    }

    /// Generic method for making a request to the Ipfs server, and getting
    /// back a response with no body.
    ///
    fn request_empty<Req>(&self, req: &Req) -> AsyncResponse<()>
    where
        Req: ApiRequest + Serialize,
    {
        let res = self.request_raw(req).and_then(
            |(status, chunk)| match status {
                StatusCode::Ok => Ok(()),
                _ => Err(Self::build_error_from_body(chunk)),
            },
        );

        Box::new(res)
    }

    /// Generic method for making a request to the Ipfs server, and getting
    /// back raw bytes.
    ///
    fn request_bytes<Req>(&self, req: &Req) -> AsyncResponse<Vec<u8>>
    where
        Req: ApiRequest + Serialize,
    {
        let res = self.request_raw(req).and_then(
            |(status, chunk)| match status {
                StatusCode::Ok => Ok(chunk.to_vec()),
                _ => Err(Self::build_error_from_body(chunk)),
            },
        );

        Box::new(res)
    }

    /// Generic method for making a request to the Ipfs server, and getting
    /// back a raw String response.
    ///
    fn request_string<Req>(&self, req: &Req) -> AsyncResponse<String>
    where
        Req: ApiRequest + Serialize,
    {
        let res = self.request_raw(req).and_then(
            |(status, chunk)| match status {
                StatusCode::Ok => String::from_utf8(chunk.to_vec()).map_err(From::from),
                _ => Err(Self::build_error_from_body(chunk)),
            },
        );

        Box::new(res)
    }

    /// Generic method to return a streaming response of deserialized json
    /// objects delineated by new line separators.
    ///
    fn request_stream<Req, Res>(&self, req: &Req) -> AsyncStreamResponse<Res>
    where
        Req: ApiRequest + Serialize,
        for<'de> Res: 'static + Deserialize<'de>,
    {
        let res = self.build_base_request(req)
            .map(|req| self.client.request(req).from_err())
            .into_future()
            .flatten()
            .map(|res| {
                IpfsClient::process_stream_response(res, JsonLineDecoder::new())
            })
            .flatten_stream();

        Box::new(res)
    }
}

impl IpfsClient {
    /// Add file to Ipfs.
    ///
    #[inline]
    pub fn add<R>(&self, data: R) -> AsyncResponse<response::AddResponse>
    where
        R: 'static + Read + Send,
    {
        self.request_with_body(data, &request::Add)
    }

    /// Returns the current ledger for a peer.
    ///
    #[inline]
    pub fn bitswap_ledger(&self, peer: &str) -> AsyncResponse<response::BitswapLedgerResponse> {
        self.request(&request::BitswapLedger { peer })
    }

    /// Returns some stats about the bitswap agent.
    ///
    #[inline]
    pub fn bitswap_stat(&self) -> AsyncResponse<response::BitswapStatResponse> {
        self.request(&request::BitswapStat)
    }

    /// Remove a given block from your wantlist.
    ///
    #[inline]
    pub fn bitswap_unwant(&self, key: &str) -> AsyncResponse<response::BitswapUnwantResponse> {
        self.request_empty(&request::BitswapUnwant { key })
    }

    /// Shows blocks on the wantlist for you or the specified peer.
    ///
    #[inline]
    pub fn bitswap_wantlist(
        &self,
        peer: Option<&str>,
    ) -> AsyncResponse<response::BitswapWantlistResponse> {
        self.request(&request::BitswapWantlist { peer })
    }

    /// Gets a raw IPFS block.
    ///
    #[inline]
    pub fn block_get(&self, hash: &str) -> AsyncResponse<response::BlockGetResponse> {
        self.request_bytes(&request::BlockGet { hash })
    }

    // TODO
    // pub fn block_put(&self, ...) -> AsyncResponse<response::BlockPutResponse> {
    // }

    /// Removes an IPFS block.
    ///
    #[inline]
    pub fn block_rm(&self, hash: &str) -> AsyncResponse<response::BlockRmResponse> {
        self.request(&request::BlockRm { hash })
    }

    /// Prints information about a raw IPFS block.
    ///
    #[inline]
    pub fn block_stat(&self, hash: &str) -> AsyncResponse<response::BlockStatResponse> {
        self.request(&request::BlockStat { hash })
    }

    /// Add default peers to the bootstrap list.
    ///
    #[inline]
    pub fn bootstrap_add_default(&self) -> AsyncResponse<response::BootstrapAddDefaultResponse> {
        self.request(&request::BootstrapAddDefault)
    }

    /// Lists peers in bootstrap list.
    ///
    #[inline]
    pub fn bootstrap_list(&self) -> AsyncResponse<response::BootstrapListResponse> {
        self.request(&request::BootstrapList)
    }

    /// Removes all peers in bootstrap list.
    ///
    #[inline]
    pub fn bootstrap_rm_all(&self) -> AsyncResponse<response::BootstrapRmAllResponse> {
        self.request(&request::BootstrapRmAll)
    }

    /// Returns the contents of an Ipfs object.
    ///
    #[inline]
    pub fn cat(&self, path: &str) -> AsyncResponse<response::CatResponse> {
        self.request_bytes(&request::Cat { path })
    }

    /// List available commands that the server accepts.
    ///
    #[inline]
    pub fn commands(&self) -> AsyncResponse<response::CommandsResponse> {
        self.request(&request::Commands)
    }

    /// Opens the config file for editing (on the server).
    ///
    #[inline]
    pub fn config_edit(&self) -> AsyncResponse<response::ConfigEditResponse> {
        self.request(&request::ConfigEdit)
    }

    // TODO
    // pub fn config_replace(&self, ...) -> AsyncResponse<response::ConfigReplaceResponse> {
    // }

    /// Show the current config of the server.
    ///
    /// Returns an unparsed json string, due to an unclear spec.
    ///
    #[inline]
    pub fn config_show(&self) -> AsyncResponse<response::ConfigShowResponse> {
        self.request_string(&request::ConfigShow)
    }

    /// Returns information about a dag node in Ipfs.
    ///
    #[inline]
    pub fn dag_get(&self, path: &str) -> AsyncResponse<response::DagGetResponse> {
        self.request(&request::DagGet { path })
    }

    // TODO
    // pub fn dag_put(&self, ...) -> AsyncResponse<response::DagPutResponse> {
    // }

    /// Query the DHT for all of the multiaddresses associated with a Peer ID.
    ///
    #[inline]
    pub fn dht_findpeer(&self, peer: &str) -> AsyncStreamResponse<response::DhtFindPeerResponse> {
        self.request_stream(&request::DhtFindPeer { peer })
    }

    /// Find peers in the DHT that can provide a specific value given a key.
    ///
    #[inline]
    pub fn dht_findprovs(&self, key: &str) -> AsyncStreamResponse<response::DhtFindProvsResponse> {
        self.request_stream(&request::DhtFindProvs { key })
    }

    /// Query the DHT for a given key.
    ///
    #[inline]
    pub fn dht_get(&self, key: &str) -> AsyncStreamResponse<response::DhtGetResponse> {
        self.request_stream(&request::DhtGet { key })
    }

    /// Announce to the network that you are providing a given value.
    ///
    #[inline]
    pub fn dht_provide(&self, key: &str) -> AsyncResponse<response::DhtProvideResponse> {
        self.request(&request::DhtProvide { key })
    }

    /// Write a key/value pair to the DHT.
    ///
    #[inline]
    pub fn dht_put(&self, key: &str, value: &str) -> AsyncStreamResponse<response::DhtPutResponse> {
        self.request_stream(&request::DhtPut { key, value })
    }

    /// Find the closest peer given the peer ID by querying the DHT.
    ///
    #[inline]
    pub fn dht_query(&self, peer: &str) -> AsyncStreamResponse<response::DhtQueryResponse> {
        self.request_stream(&request::DhtQuery { peer })
    }

    /// Clear inactive requests from the log.
    ///
    #[inline]
    pub fn diag_cmds_clear(&self) -> AsyncResponse<response::DiagCmdsClearResponse> {
        self.request_empty(&request::DiagCmdsClear)
    }

    /// Set how long to keep inactive requests in the log.
    ///
    #[inline]
    pub fn diag_cmds_set_time(
        &self,
        time: &str,
    ) -> AsyncResponse<response::DiagCmdsSetTimeResponse> {
        self.request_empty(&request::DiagCmdsSetTime { time })
    }

    /// Print system diagnostic information.
    ///
    /// Note: There isn't good documentation on what this call is supposed to return.
    /// It might be platform dependent, but if it isn't, this can be fixed to return
    /// an actual object.
    ///
    #[inline]
    pub fn diag_sys(&self) -> AsyncResponse<response::DiagSysResponse> {
        self.request_string(&request::DiagSys)
    }

    /// Resolve DNS link.
    ///
    #[inline]
    pub fn dns(&self, link: &str, recursive: bool) -> AsyncResponse<response::DnsResponse> {
        self.request(&request::Dns { link, recursive })
    }

    /// List directory for Unix filesystem objects.
    ///
    #[inline]
    pub fn file_ls(&self, path: &str) -> AsyncResponse<response::FileLsResponse> {
        self.request(&request::FileLs { path })
    }

    /// Copy files into MFS.
    ///
    #[inline]
    pub fn files_cp(&self, path: &str, dest: &str) -> AsyncResponse<response::FilesCpResponse> {
        self.request_empty(&request::FilesCp { path, dest })
    }

    /// Flush a path's data to disk.
    ///
    #[inline]
    pub fn files_flush(&self, path: Option<&str>) -> AsyncResponse<response::FilesFlushResponse> {
        self.request_empty(&request::FilesFlush { path })
    }

    /// List directories in MFS.
    ///
    #[inline]
    pub fn files_ls(&self, path: Option<&str>) -> AsyncResponse<response::FilesLsResponse> {
        self.request(&request::FilesLs { path })
    }

    /// Make directories in MFS.
    ///
    #[inline]
    pub fn files_mkdir(
        &self,
        path: &str,
        parents: bool,
    ) -> AsyncResponse<response::FilesMkdirResponse> {
        self.request_empty(&request::FilesMkdir { path, parents })
    }

    /// Copy files into MFS.
    ///
    #[inline]
    pub fn files_mv(&self, path: &str, dest: &str) -> AsyncResponse<response::FilesMvResponse> {
        self.request_empty(&request::FilesMv { path, dest })
    }

    /// Read a file in MFS.
    ///
    #[inline]
    pub fn files_read(&self, path: &str) -> AsyncResponse<response::FilesReadResponse> {
        self.request_bytes(&request::FilesRead { path })
    }

    /// Remove a file in MFS.
    ///
    #[inline]
    pub fn files_rm(
        &self,
        path: &str,
        recursive: bool,
    ) -> AsyncResponse<response::FilesRmResponse> {
        self.request_empty(&request::FilesRm { path, recursive })
    }

    /// Display a file's status in MDFS.
    ///
    #[inline]
    pub fn files_stat(&self, path: &str) -> AsyncResponse<response::FilesStatResponse> {
        self.request(&request::FilesStat { path })
    }

    // TODO
    // pub fn files_write(&self, ...) -> AsyncResponse<response::FilesWriteResponse> {
    // }

    /// List blocks that are both in the filestore and standard block storage.
    ///
    #[inline]
    pub fn filestore_dups(&self) -> AsyncStreamResponse<response::FilestoreDupsResponse> {
        self.request_stream(&request::FilestoreDups)
    }

    /// List objects in filestore.
    ///
    #[inline]
    pub fn filestore_ls(&self) -> AsyncStreamResponse<response::FilestoreLsResponse> {
        self.request_stream(&request::FilestoreLs)
    }

    /// Verify objects in filestore.
    ///
    #[inline]
    pub fn filestore_verify(&self) -> AsyncStreamResponse<response::FilestoreVerifyResponse> {
        self.request_stream(&request::FilestoreVerify)
    }

    /// Download Ipfs object.
    ///
    #[inline]
    pub fn get(&self, path: &str) -> AsyncResponse<response::GetResponse> {
        self.request_bytes(&request::Get { path })
    }

    /// Returns information about a peer.
    ///
    /// If `peer` is `None`, returns information about you.
    ///
    #[inline]
    pub fn id(&self, peer: Option<&str>) -> AsyncResponse<response::IdResponse> {
        self.request(&request::Id { peer })
    }

    /// Create a new keypair.
    ///
    #[inline]
    pub fn key_gen(
        &self,
        name: &str,
        kind: request::KeyType,
        size: Option<i32>,
    ) -> AsyncResponse<response::KeyGenResponse> {
        self.request(&request::KeyGen { name, kind, size })
    }

    /// List all local keypairs.
    ///
    #[inline]
    pub fn key_list(&self) -> AsyncResponse<response::KeyListResponse> {
        self.request(&request::KeyList)
    }

    /// Change the logging level for a logger.
    ///
    #[inline]
    pub fn log_level(
        &self,
        logger: request::Logger,
        level: request::LoggingLevel,
    ) -> AsyncResponse<response::LogLevelResponse> {
        self.request(&request::LogLevel { logger, level })
    }

    /// List all logging subsystems.
    ///
    #[inline]
    pub fn log_ls(&self) -> AsyncResponse<response::LogLsResponse> {
        self.request(&request::LogLs)
    }

    /// Read the event log.
    ///
    pub fn log_tail(&self) -> AsyncStreamResponse<String> {
        let res = self.build_base_request(&request::LogTail)
            .map(|req| self.client.request(req).from_err())
            .into_future()
            .flatten()
            .map(|res| IpfsClient::process_stream_response(res, LineDecoder))
            .flatten_stream();

        Box::new(res)
    }

    /// List the contents of an Ipfs multihash.
    ///
    #[inline]
    pub fn ls(&self, path: Option<&str>) -> AsyncResponse<response::LsResponse> {
        self.request(&request::Ls { path })
    }

    /// Returns the diff of two Ipfs objects.
    ///
    #[inline]
    pub fn object_diff(
        &self,
        key0: &str,
        key1: &str,
    ) -> AsyncResponse<response::ObjectDiffResponse> {
        self.request(&request::ObjectDiff { key0, key1 })
    }

    /// Returns the data in an object.
    ///
    #[inline]
    pub fn object_get(&self, key: &str) -> AsyncResponse<response::ObjectGetResponse> {
        self.request(&request::ObjectGet { key })
    }

    /// Returns the links that an object points to.
    ///
    #[inline]
    pub fn object_links(&self, key: &str) -> AsyncResponse<response::ObjectLinksResponse> {
        self.request(&request::ObjectLinks { key })
    }

    /// Returns the stats for an object.
    ///
    #[inline]
    pub fn object_stat(&self, key: &str) -> AsyncResponse<response::ObjectStatResponse> {
        self.request(&request::ObjectStat { key })
    }

    /// Returns a list of pinned objects in local storage.
    ///
    #[inline]
    pub fn pin_ls(
        &self,
        key: Option<&str>,
        typ: Option<&str>,
    ) -> AsyncResponse<response::PinLsResponse> {
        self.request(&request::PinLs { key, typ })
    }

    /// Removes a pinned object from local storage.
    ///
    #[inline]
    pub fn pin_rm(
        &self,
        key: &str,
        recursive: Option<bool>,
    ) -> AsyncResponse<response::PinRmResponse> {
        self.request(&request::PinRm { key, recursive })
    }

    /// Pings a peer.
    ///
    #[inline]
    pub fn ping(
        &self,
        peer: &str,
        count: Option<usize>,
    ) -> AsyncStreamResponse<response::PingResponse> {
        self.request_stream(&request::Ping { peer, count })
    }

    /// List subscribed pubsub topics.
    ///
    #[inline]
    pub fn pubsub_ls(&self) -> AsyncResponse<response::PubsubLsResponse> {
        self.request(&request::PubsubLs)
    }

    /// List peers that are being published to.
    ///
    #[inline]
    pub fn pubsub_peers(
        &self,
        topic: Option<&str>,
    ) -> AsyncResponse<response::PubsubPeersResponse> {
        self.request(&request::PubsubPeers { topic })
    }

    /// Publish a message to a topic.
    ///
    #[inline]
    pub fn pubsub_pub(
        &self,
        topic: &str,
        payload: &str,
    ) -> AsyncResponse<response::PubsubPubResponse> {
        self.request_empty(&request::PubsubPub { topic, payload })
    }

    /// Subscribes to a pubsub topic.
    ///
    #[inline]
    pub fn pubsub_sub(
        &self,
        topic: &str,
        discover: Option<bool>,
    ) -> AsyncStreamResponse<response::PubsubSubResponse> {
        self.request_stream(&request::PubsubSub { topic, discover })
    }

    /// Gets a list of local references.
    ///
    #[inline]
    pub fn refs_local(&self) -> AsyncStreamResponse<response::RefsLocalResponse> {
        self.request_stream(&request::RefsLocal)
    }

    /// Returns bitswap stats.
    ///
    #[inline]
    pub fn stats_bitswap(&self) -> AsyncResponse<response::StatsBitswapResponse> {
        self.request(&request::StatsBitswap)
    }

    /// Returns bandwidth stats.
    ///
    #[inline]
    pub fn stats_bw(&self) -> AsyncResponse<response::StatsBwResponse> {
        self.request(&request::StatsBw)
    }

    /// Returns repo stats.
    ///
    #[inline]
    pub fn stats_repo(&self) -> AsyncResponse<response::StatsRepoResponse> {
        self.request(&request::StatsRepo)
    }

    /// Return a list of local addresses.
    ///
    #[inline]
    pub fn swarm_addrs_local(&self) -> AsyncResponse<response::SwarmAddrsLocalResponse> {
        self.request(&request::SwarmAddrsLocal)
    }

    /// Return a list of peers with open connections.
    ///
    #[inline]
    pub fn swarm_peers(&self) -> AsyncResponse<response::SwarmPeersResponse> {
        self.request(&request::SwarmPeers)
    }

    /// Returns information about the Ipfs server version.
    ///
    #[inline]
    pub fn version(&self) -> AsyncResponse<response::VersionResponse> {
        self.request(&request::Version)
    }
}
