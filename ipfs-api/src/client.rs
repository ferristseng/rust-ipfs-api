// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use futures::{
    stream::{self, Stream},
    Future, IntoFuture,
    future,
};
use header::TRAILER;
use http::uri::InvalidUri;
use hyper::{
    self,
    client::{Client, HttpConnector},
    Chunk, Request, Response, StatusCode, Uri,
};
use hyper_multipart::client::multipart;
use read::{JsonLineDecoder, LineDecoder, StreamReader};
use request::{self, ApiRequest};
use response::{self, Error};
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::Read;
use std::path::{Path, PathBuf};
use tokio_codec::{Decoder, FramedRead};

/// A response returned by the HTTP client.
///
type AsyncResponse<T> = Box<Future<Item = T, Error = Error> + Send + 'static>;

/// A future that returns a stream of responses.
///
type AsyncStreamResponse<T> = Box<Stream<Item = T, Error = Error> + Send + 'static>;

/// Asynchronous Ipfs client.
///
#[derive(Clone)]
pub struct IpfsClient {
    base: Uri,
    client: Client<HttpConnector, hyper::Body>,
}

impl Default for IpfsClient {
    /// Creates an `IpfsClient` connected to `localhost:5001`.
    ///
    fn default() -> IpfsClient {
        IpfsClient::new("localhost", 5001).unwrap()
    }
}

impl IpfsClient {
    /// Creates a new `IpfsClient`.
    ///
    #[inline]
    pub fn new(host: &str, port: u16) -> Result<IpfsClient, InvalidUri> {
        let base_path = IpfsClient::build_base_path(host, port)?;

        Ok(IpfsClient {
            base: base_path,
            client: Client::builder().keep_alive(false).build_http(),
        })
    }

    /// Builds the base url path for the Ipfs api.
    ///
    fn build_base_path(host: &str, port: u16) -> Result<Uri, InvalidUri> {
        format!("http://{}:{}/api/v0", host, port).parse()
    }

    /// Builds the url for an api call.
    ///
    fn build_base_request<Req>(
        &self,
        req: &Req,
        form: Option<multipart::Form>,
    ) -> Result<Request<hyper::Body>, Error>
    where
        Req: ApiRequest + Serialize,
    {
        let url = format!(
            "{}{}?{}",
            self.base,
            Req::PATH,
            ::serde_urlencoded::to_string(req)?
        );

        url.parse::<Uri>().map_err(From::from).and_then(move |url| {
            let mut builder = Request::builder();
            let mut builder = builder.method(Req::METHOD.clone()).uri(url);

            let req = if let Some(form) = form {
                form.set_body(&mut builder)
            } else {
                builder.body(hyper::Body::empty())
            };

            req.map_err(From::from)
        })
    }

    /// Builds an Api error from a response body.
    ///
    #[inline]
    fn build_error_from_body(chunk: Chunk) -> Error {
        match serde_json::from_slice(&chunk) {
            Ok(e) => Error::Api(e).into(),
            Err(_) => match String::from_utf8(chunk.to_vec()) {
                Ok(s) => Error::Uncategorized(s).into(),
                Err(e) => e.into(),
            },
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
            StatusCode::OK => serde_json::from_slice(&chunk).map_err(From::from),
            _ => Err(Self::build_error_from_body(chunk)),
        }
    }

    /// Processes a response that returns a stream of json deserializable
    /// results.
    ///
    fn process_stream_response<D, Res>(
        res: Response<hyper::Body>,
        decoder: D,
    ) -> AsyncStreamResponse<Res>
    where
        D: 'static + Decoder<Item = Res, Error = Error> + Send,
        Res: 'static,
    {
        let stream = FramedRead::new(StreamReader::new(res.into_body().from_err()), decoder);

        Box::new(stream)
    }

    /// Generates a request, and returns the unprocessed response future.
    ///
    fn request_raw<Req>(
        &self,
        req: &Req,
        form: Option<multipart::Form>,
    ) -> AsyncResponse<(StatusCode, Chunk)>
    where
        Req: ApiRequest + Serialize,
    {
        match self.build_base_request(req, form) {
            Ok(req) => {
                let res = self
                    .client
                    .request(req)
                    .and_then(|res| {
                        let status = res.status();

                        res.into_body().concat2().map(move |chunk| (status, chunk))
                    }).from_err();

                Box::new(res)
            }
            Err(e) => Box::new(Err(e).into_future()),
        }
    }

    /// Generic method for making a request that expects back a streaming
    /// response.
    ///
    fn request_stream<Req, Res, F>(
        &self,
        req: &Req,
        form: Option<multipart::Form>,
        process: F,
    ) -> AsyncStreamResponse<Res>
    where
        Req: ApiRequest + Serialize,
        Res: 'static + Send,
        F: 'static + Fn(hyper::Response<hyper::Body>) -> AsyncStreamResponse<Res> + Send,
    {
        match self.build_base_request(req, form) {
            Ok(req) => {
                let res = self
                    .client
                    .request(req)
                    .from_err()
                    .map(move |res| {
                        let stream: Box<
                            Stream<Item = Res, Error = _> + Send + 'static,
                        > = match res.status() {
                            StatusCode::OK => process(res),
                            // If the server responded with an error status code, the body
                            // still needs to be read so an error can be built. This block will
                            // read the entire body stream, then immediately return an error.
                            //
                            _ => Box::new(
                                res.into_body()
                                    .concat2()
                                    .from_err()
                                    .and_then(|chunk| Err(Self::build_error_from_body(chunk)))
                                    .into_stream(),
                            ),
                        };

                        stream
                    }).flatten_stream();

                Box::new(res)
            }
            Err(e) => Box::new(stream::once(Err(e))),
        }
    }

    /// Generic method for making a request to the Ipfs server, and getting
    /// a deserializable response.
    ///
    fn request<Req, Res>(&self, req: &Req, form: Option<multipart::Form>) -> AsyncResponse<Res>
    where
        Req: ApiRequest + Serialize,
        for<'de> Res: 'static + Deserialize<'de> + Send,
    {
        let res = self
            .request_raw(req, form)
            .and_then(|(status, chunk)| IpfsClient::process_json_response(status, chunk));

        Box::new(res)
    }

    /// Generic method for making a request to the Ipfs server, and getting
    /// back a response with no body.
    ///
    fn request_empty<Req>(&self, req: &Req, form: Option<multipart::Form>) -> AsyncResponse<()>
    where
        Req: ApiRequest + Serialize,
    {
        let res = self
            .request_raw(req, form)
            .and_then(|(status, chunk)| match status {
                StatusCode::OK => Ok(()),
                _ => Err(Self::build_error_from_body(chunk)),
            });

        Box::new(res)
    }

    /// Generic method for making a request to the Ipfs server, and getting
    /// back a raw String response.
    ///
    fn request_string<Req>(&self, req: &Req, form: Option<multipart::Form>) -> AsyncResponse<String>
    where
        Req: ApiRequest + Serialize,
    {
        let res = self
            .request_raw(req, form)
            .and_then(|(status, chunk)| match status {
                StatusCode::OK => String::from_utf8(chunk.to_vec()).map_err(From::from),
                _ => Err(Self::build_error_from_body(chunk)),
            });

        Box::new(res)
    }

    /// Generic method for making a request to the Ipfs server, and getting
    /// back a raw stream of bytes.
    ///
    fn request_stream_bytes<Req>(
        &self,
        req: &Req,
        form: Option<multipart::Form>,
    ) -> AsyncStreamResponse<Chunk>
    where
        Req: ApiRequest + Serialize,
    {
        self.request_stream(req, form, |res| Box::new(res.into_body().from_err()))
    }

    /// Generic method to return a streaming response of deserialized json
    /// objects delineated by new line separators.
    ///
    fn request_stream_json<Req, Res>(
        &self,
        req: &Req,
        form: Option<multipart::Form>,
    ) -> AsyncStreamResponse<Res>
    where
        Req: ApiRequest + Serialize,
        for<'de> Res: 'static + Deserialize<'de> + Send,
    {
        self.request_stream(req, form, |res| {
            let parse_stream_error = if let Some(trailer) = res.headers().get(TRAILER) {
                // Response has the Trailer header set. The StreamError trailer
                // is used to indicate that there was an error while streaming
                // data with Ipfs.
                //
                if trailer == "X-Stream-Error" {
                    true
                } else {
                    let err = Error::UnrecognizedTrailerHeader(
                        String::from_utf8_lossy(trailer.as_ref()).into(),
                    );

                    // There was an unrecognized trailer value. If that is the case,
                    // create a stream that immediately errors.
                    //
                    return Box::new(stream::once(Err(err.into())));
                }
            } else {
                false
            };

            Box::new(IpfsClient::process_stream_response(
                res,
                JsonLineDecoder::new(parse_stream_error),
            ))
        })
    }
}

impl IpfsClient {
    /// Add file to Ipfs.
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
    /// let req = client.add(data);
    /// # }
    /// ```
    ///
    #[inline]
    pub fn add<R>(&self, data: R) -> AsyncResponse<response::AddResponse>
    where
        R: 'static + Read + Send,
    {
        let mut form = multipart::Form::default();

        form.add_reader("path", data);

        self.request(&request::Add, Some(form))
    }

    /// Add a path to Ipfs. Can be a file or directory.
    /// A hard limit of 128 open file descriptors is set such
    /// that any small additional files are stored in-memory.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let path = "./src";
    /// let req = client.add_path(path);
    /// # }
    /// ```
    ///
    #[inline]
    pub fn add_path<P>(&self, path: P) -> AsyncResponse<response::AddResponse>
    where
        P: AsRef<Path>,
    {
        let mut form = multipart::Form::default();

        let prefix = path.as_ref().parent();

        let mut paths_to_add: Vec<(PathBuf, u64)> = vec![];

        for path in walkdir::WalkDir::new(path.as_ref()) {
            match path {
                Ok(entry) => {
                    if entry.file_type().is_file() {
                        let file_size =
                            entry.metadata().map(|metadata| metadata.len()).unwrap_or(0);
                        paths_to_add.push((entry.path().to_path_buf(), file_size));
                    }
                }
                Err(err) => {
                    return Box::new(future::err(Error::Io(err.into())));
                }
            }
        }

        paths_to_add.sort_unstable_by(|(_, a), (_, b)| a.cmp(b).reverse());

        let mut it = 0;
        const FILE_DESCRIPTOR_LIMIT: usize = 127;

        for (path, file_size) in paths_to_add {
            let file = std::fs::File::open(&path);
            if file.is_err() {
                return Box::new(future::err(file.unwrap_err().into()));
            }
            let file_name = match prefix {
                Some(prefix) => path.strip_prefix(prefix).unwrap(),
                None => path.as_path(),
            }
            .to_string_lossy();

            if it < FILE_DESCRIPTOR_LIMIT {
                form.add_reader_file("path", file.unwrap(), file_name);
                it += 1;
            } else {
                let mut buf = Vec::with_capacity(file_size as usize);
                if let Err(err) = file.unwrap().read_to_end(&mut buf) {
                    return Box::new(future::err(err.into()));
                }
                form.add_reader_file("path", std::io::Cursor::new(buf), file_name);
            }
        }

        Box::new(
            self.request_stream_json(&request::Add, Some(form))
                .collect()
                .map(|mut responses: Vec<response::AddResponse>| responses.pop().unwrap()),
        )
    }

    /// Returns the current ledger for a peer.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.bitswap_ledger("QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ");
    /// # }
    /// ```
    ///
    #[inline]
    pub fn bitswap_ledger(&self, peer: &str) -> AsyncResponse<response::BitswapLedgerResponse> {
        self.request(&request::BitswapLedger { peer }, None)
    }

    /// Triggers a reprovide.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.bitswap_reprovide();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn bitswap_reprovide(&self) -> AsyncResponse<response::BitswapReprovideResponse> {
        self.request_empty(&request::BitswapReprovide, None)
    }

    /// Returns some stats about the bitswap agent.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.bitswap_stat();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn bitswap_stat(&self) -> AsyncResponse<response::BitswapStatResponse> {
        self.request(&request::BitswapStat, None)
    }

    /// Remove a given block from your wantlist.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.bitswap_unwant("QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA");
    /// # }
    /// ```
    ///
    #[inline]
    pub fn bitswap_unwant(&self, key: &str) -> AsyncResponse<response::BitswapUnwantResponse> {
        self.request_empty(&request::BitswapUnwant { key }, None)
    }

    /// Shows blocks on the wantlist for you or the specified peer.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.bitswap_wantlist(Some("QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ"));
    /// # }
    /// ```
    ///
    #[inline]
    pub fn bitswap_wantlist(
        &self,
        peer: Option<&str>,
    ) -> AsyncResponse<response::BitswapWantlistResponse> {
        self.request(&request::BitswapWantlist { peer }, None)
    }

    /// Gets a raw IPFS block.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate futures;
    /// # extern crate ipfs_api;
    /// #
    /// use futures::Stream;
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let hash = "QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA";
    /// let req = client.block_get(hash).concat2();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn block_get(&self, hash: &str) -> AsyncStreamResponse<Chunk> {
        self.request_stream_bytes(&request::BlockGet { hash }, None)
    }

    /// Store input as an IPFS block.
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
    /// let req = client.block_put(data);
    /// # }
    /// ```
    ///
    #[inline]
    pub fn block_put<R>(&self, data: R) -> AsyncResponse<response::BlockPutResponse>
    where
        R: 'static + Read + Send,
    {
        let mut form = multipart::Form::default();

        form.add_reader("data", data);

        self.request(&request::BlockPut, Some(form))
    }

    /// Removes an IPFS block.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.block_rm("QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA");
    /// # }
    /// ```
    ///
    #[inline]
    pub fn block_rm(&self, hash: &str) -> AsyncResponse<response::BlockRmResponse> {
        self.request(&request::BlockRm { hash }, None)
    }

    /// Prints information about a raw IPFS block.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.block_stat("QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA");
    /// # }
    /// ```
    ///
    #[inline]
    pub fn block_stat(&self, hash: &str) -> AsyncResponse<response::BlockStatResponse> {
        self.request(&request::BlockStat { hash }, None)
    }

    /// Add default peers to the bootstrap list.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.bootstrap_add_default();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn bootstrap_add_default(&self) -> AsyncResponse<response::BootstrapAddDefaultResponse> {
        self.request(&request::BootstrapAddDefault, None)
    }

    /// Lists peers in bootstrap list.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.bootstrap_list();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn bootstrap_list(&self) -> AsyncResponse<response::BootstrapListResponse> {
        self.request(&request::BootstrapList, None)
    }

    /// Removes all peers in bootstrap list.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.bootstrap_rm_all();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn bootstrap_rm_all(&self) -> AsyncResponse<response::BootstrapRmAllResponse> {
        self.request(&request::BootstrapRmAll, None)
    }

    /// Returns the contents of an Ipfs object.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate futures;
    /// # extern crate ipfs_api;
    /// #
    /// use futures::Stream;
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let hash = "QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA";
    /// let req = client.cat(hash).concat2();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn cat(&self, path: &str) -> AsyncStreamResponse<Chunk> {
        self.request_stream_bytes(&request::Cat { path }, None)
    }

    /// List available commands that the server accepts.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.commands();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn commands(&self) -> AsyncResponse<response::CommandsResponse> {
        self.request(&request::Commands, None)
    }

    /// Opens the config file for editing (on the server).
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.config_edit();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn config_edit(&self) -> AsyncResponse<response::ConfigEditResponse> {
        self.request(&request::ConfigEdit, None)
    }

    /// Replace the config file.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    /// use std::io::Cursor;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let config = Cursor::new("{..json..}");
    /// let req = client.config_replace(config);
    /// # }
    /// ```
    ///
    #[inline]
    pub fn config_replace<R>(&self, data: R) -> AsyncResponse<response::ConfigReplaceResponse>
    where
        R: 'static + Read + Send,
    {
        let mut form = multipart::Form::default();

        form.add_reader("file", data);

        self.request_empty(&request::ConfigReplace, Some(form))
    }

    /// Show the current config of the server.
    ///
    /// Returns an unparsed json string, due to an unclear spec.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.config_show();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn config_show(&self) -> AsyncResponse<response::ConfigShowResponse> {
        self.request_string(&request::ConfigShow, None)
    }

    /// Returns information about a dag node in Ipfs.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.dag_get("QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA");
    /// # }
    /// ```
    ///
    #[inline]
    pub fn dag_get(&self, path: &str) -> AsyncResponse<response::DagGetResponse> {
        self.request(&request::DagGet { path }, None)
    }

    // TODO /dag routes are experimental, and there isn't a whole lot of
    // documentation available for how this route works.
    //
    // /// Add a DAG node to Ipfs.
    // ///
    // #[inline]
    // pub fn dag_put<R>(&self, data: R) -> AsyncResponse<response::DagPutResponse>
    // where
    //     R: 'static + Read + Send,
    // {
    //     let mut form = multipart::Form::default();
    //
    //     form.add_reader("arg", data);
    //
    //     self.request(&request::DagPut, Some(form))
    // }

    // TODO /dag/resolve

    /// Query the DHT for all of the multiaddresses associated with a Peer ID.
    ///
    /// ```no_run
    /// # extern crate futures;
    /// # extern crate ipfs_api;
    /// #
    /// use futures::Stream;
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let peer = "QmSoLPppuBtQSGwKDZT2M73ULpjvfd3aZ6ha4oFGL1KrGM";
    /// let req = client.dht_findpeer(peer).collect();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn dht_findpeer(&self, peer: &str) -> AsyncStreamResponse<response::DhtFindPeerResponse> {
        self.request_stream_json(&request::DhtFindPeer { peer }, None)
    }

    /// Find peers in the DHT that can provide a specific value given a key.
    ///
    /// ```no_run
    /// # extern crate futures;
    /// # extern crate ipfs_api;
    /// #
    /// use futures::Stream;
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let key = "QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA";
    /// let req = client.dht_findprovs(key).collect();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn dht_findprovs(&self, key: &str) -> AsyncStreamResponse<response::DhtFindProvsResponse> {
        self.request_stream_json(&request::DhtFindProvs { key }, None)
    }

    /// Query the DHT for a given key.
    ///
    /// ```no_run
    /// # extern crate futures;
    /// # extern crate ipfs_api;
    /// #
    /// use futures::Stream;
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let key = "QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA";
    /// let req = client.dht_get(key).collect();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn dht_get(&self, key: &str) -> AsyncStreamResponse<response::DhtGetResponse> {
        self.request_stream_json(&request::DhtGet { key }, None)
    }

    /// Announce to the network that you are providing a given value.
    ///
    /// ```no_run
    /// # extern crate futures;
    /// # extern crate ipfs_api;
    /// #
    /// use futures::Stream;
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let key = "QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA";
    /// let req = client.dht_provide(key).collect();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn dht_provide(&self, key: &str) -> AsyncStreamResponse<response::DhtProvideResponse> {
        self.request_stream_json(&request::DhtProvide { key }, None)
    }

    /// Write a key/value pair to the DHT.
    ///
    /// ```no_run
    /// # extern crate futures;
    /// # extern crate ipfs_api;
    /// #
    /// use futures::Stream;
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.dht_put("test", "Hello World!").collect();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn dht_put(&self, key: &str, value: &str) -> AsyncStreamResponse<response::DhtPutResponse> {
        self.request_stream_json(&request::DhtPut { key, value }, None)
    }

    /// Find the closest peer given the peer ID by querying the DHT.
    ///
    /// ```no_run
    /// # extern crate futures;
    /// # extern crate ipfs_api;
    /// #
    /// use futures::Stream;
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let peer = "QmSoLPppuBtQSGwKDZT2M73ULpjvfd3aZ6ha4oFGL1KrGM";
    /// let req = client.dht_query(peer).collect();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn dht_query(&self, peer: &str) -> AsyncStreamResponse<response::DhtQueryResponse> {
        self.request_stream_json(&request::DhtQuery { peer }, None)
    }

    /// Clear inactive requests from the log.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.diag_cmds_clear();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn diag_cmds_clear(&self) -> AsyncResponse<response::DiagCmdsClearResponse> {
        self.request_empty(&request::DiagCmdsClear, None)
    }

    /// Set how long to keep inactive requests in the log.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.diag_cmds_set_time("1m");
    /// # }
    /// ```
    ///
    #[inline]
    pub fn diag_cmds_set_time(
        &self,
        time: &str,
    ) -> AsyncResponse<response::DiagCmdsSetTimeResponse> {
        self.request_empty(&request::DiagCmdsSetTime { time }, None)
    }

    /// Print system diagnostic information.
    ///
    /// Note: There isn't good documentation on what this call is supposed to return.
    /// It might be platform dependent, but if it isn't, this can be fixed to return
    /// an actual object.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.diag_sys();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn diag_sys(&self) -> AsyncResponse<response::DiagSysResponse> {
        self.request_string(&request::DiagSys, None)
    }

    /// Resolve DNS link.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.dns("ipfs.io", true);
    /// # }
    /// ```
    ///
    #[inline]
    pub fn dns(&self, link: &str, recursive: bool) -> AsyncResponse<response::DnsResponse> {
        self.request(&request::Dns { link, recursive }, None)
    }

    /// List directory for Unix filesystem objects.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.file_ls("/ipns/ipfs.io");
    /// # }
    /// ```
    ///
    #[inline]
    pub fn file_ls(&self, path: &str) -> AsyncResponse<response::FileLsResponse> {
        self.request(&request::FileLs { path }, None)
    }

    /// Copy files into MFS.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.files_cp("/path/to/file", "/dest");
    /// # }
    /// ```
    ///
    #[inline]
    pub fn files_cp(&self, path: &str, dest: &str) -> AsyncResponse<response::FilesCpResponse> {
        self.request_empty(&request::FilesCp { path, dest }, None)
    }

    /// Flush a path's data to disk.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.files_flush(None);
    /// let req = client.files_flush(Some("/tmp"));
    /// # }
    /// ```
    ///
    #[inline]
    pub fn files_flush(&self, path: Option<&str>) -> AsyncResponse<response::FilesFlushResponse> {
        self.request_empty(&request::FilesFlush { path }, None)
    }

    /// List directories in MFS.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.files_ls(None);
    /// let req = client.files_ls(Some("/tmp"));
    /// # }
    /// ```
    ///
    #[inline]
    pub fn files_ls(&self, path: Option<&str>) -> AsyncResponse<response::FilesLsResponse> {
        self.request(&request::FilesLs { path }, None)
    }

    /// Make directories in MFS.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.files_mkdir("/test", false);
    /// let req = client.files_mkdir("/test/nested/dir", true);
    /// # }
    /// ```
    ///
    #[inline]
    pub fn files_mkdir(
        &self,
        path: &str,
        parents: bool,
    ) -> AsyncResponse<response::FilesMkdirResponse> {
        self.request_empty(&request::FilesMkdir { path, parents }, None)
    }

    /// Copy files into MFS.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.files_mv("/test/tmp.json", "/test/file.json");
    /// # }
    /// ```
    ///
    #[inline]
    pub fn files_mv(&self, path: &str, dest: &str) -> AsyncResponse<response::FilesMvResponse> {
        self.request_empty(&request::FilesMv { path, dest }, None)
    }

    /// Read a file in MFS.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.files_read("/test/file.json");
    /// # }
    /// ```
    ///
    #[inline]
    pub fn files_read(&self, path: &str) -> AsyncStreamResponse<Chunk> {
        self.request_stream_bytes(&request::FilesRead { path }, None)
    }

    /// Remove a file in MFS.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.files_rm("/test/dir", true);
    /// let req = client.files_rm("/test/file.json", false);
    /// # }
    /// ```
    ///
    #[inline]
    pub fn files_rm(
        &self,
        path: &str,
        recursive: bool,
    ) -> AsyncResponse<response::FilesRmResponse> {
        self.request_empty(&request::FilesRm { path, recursive }, None)
    }

    /// Display a file's status in MDFS.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.files_stat("/test/file.json");
    /// # }
    /// ```
    ///
    #[inline]
    pub fn files_stat(&self, path: &str) -> AsyncResponse<response::FilesStatResponse> {
        self.request(&request::FilesStat { path }, None)
    }

    /// Write to a mutable file in the filesystem.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    /// use std::fs::File;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let file = File::open("test.json").unwrap();
    /// let req = client.files_write("/test/file.json", true, true, file);
    /// # }
    /// ```
    ///
    #[inline]
    pub fn files_write<R>(
        &self,
        path: &str,
        create: bool,
        truncate: bool,
        data: R,
    ) -> AsyncResponse<response::FilesWriteResponse>
    where
        R: 'static + Read + Send,
    {
        let mut form = multipart::Form::default();

        form.add_reader("data", data);

        self.request_empty(
            &request::FilesWrite {
                path,
                create,
                truncate,
            },
            Some(form),
        )
    }

    /// List blocks that are both in the filestore and standard block storage.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.filestore_dups();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn filestore_dups(&self) -> AsyncStreamResponse<response::FilestoreDupsResponse> {
        self.request_stream_json(&request::FilestoreDups, None)
    }

    /// List objects in filestore.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.filestore_ls(Some("QmYPP3BovR2m8UqCZxFbdXSit6SKgExxDkFAPLqiGsap4X"));
    /// # }
    /// ```
    ///
    #[inline]
    pub fn filestore_ls(
        &self,
        cid: Option<&str>,
    ) -> AsyncStreamResponse<response::FilestoreLsResponse> {
        self.request_stream_json(&request::FilestoreLs { cid }, None)
    }

    /// Verify objects in filestore.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.filestore_verify(None);
    /// # }
    /// ```
    ///
    #[inline]
    pub fn filestore_verify(
        &self,
        cid: Option<&str>,
    ) -> AsyncStreamResponse<response::FilestoreVerifyResponse> {
        self.request_stream_json(&request::FilestoreVerify { cid }, None)
    }

    /// Download Ipfs object.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.get("/test/file.json");
    /// # }
    /// ```
    ///
    #[inline]
    pub fn get(&self, path: &str) -> AsyncStreamResponse<Chunk> {
        self.request_stream_bytes(&request::Get { path }, None)
    }

    /// Returns information about a peer.
    ///
    /// If `peer` is `None`, returns information about you.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.id(None);
    /// let req = client.id(Some("QmSoLPppuBtQSGwKDZT2M73ULpjvfd3aZ6ha4oFGL1KrGM"));
    /// # }
    /// ```
    ///
    #[inline]
    pub fn id(&self, peer: Option<&str>) -> AsyncResponse<response::IdResponse> {
        self.request(&request::Id { peer }, None)
    }

    /// Create a new keypair.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::{IpfsClient, KeyType};
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.key_gen("test", KeyType::Rsa, 64);
    /// # }
    /// ```
    ///
    #[inline]
    pub fn key_gen(
        &self,
        name: &str,
        kind: request::KeyType,
        size: i32,
    ) -> AsyncResponse<response::KeyGenResponse> {
        self.request(&request::KeyGen { name, kind, size }, None)
    }

    /// List all local keypairs.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.key_list();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn key_list(&self) -> AsyncResponse<response::KeyListResponse> {
        self.request(&request::KeyList, None)
    }

    /// Rename a keypair.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.key_rename("key_0", "new_name", false);
    /// # }
    /// ```
    ///
    #[inline]
    pub fn key_rename(
        &self,
        name: &str,
        new: &str,
        force: bool,
    ) -> AsyncResponse<response::KeyRenameResponse> {
        self.request(&request::KeyRename { name, new, force }, None)
    }

    /// Remove a keypair.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.key_rm("key_0");
    /// # }
    /// ```
    ///
    #[inline]
    pub fn key_rm(&self, name: &str) -> AsyncResponse<response::KeyRmResponse> {
        self.request(&request::KeyRm { name }, None)
    }

    /// Change the logging level for a logger.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::{IpfsClient, Logger, LoggingLevel};
    /// use std::borrow::Cow;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.log_level(Logger::All, LoggingLevel::Debug);
    /// let req = client.log_level(
    ///     Logger::Specific(Cow::Borrowed("web")),
    ///     LoggingLevel::Warning);
    /// # }
    /// ```
    ///
    #[inline]
    pub fn log_level(
        &self,
        logger: request::Logger,
        level: request::LoggingLevel,
    ) -> AsyncResponse<response::LogLevelResponse> {
        self.request(&request::LogLevel { logger, level }, None)
    }

    /// List all logging subsystems.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.log_ls();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn log_ls(&self) -> AsyncResponse<response::LogLsResponse> {
        self.request(&request::LogLs, None)
    }

    /// Read the event log.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.log_tail();
    /// # }
    /// ```
    ///
    pub fn log_tail(&self) -> AsyncStreamResponse<String> {
        let res = self
            .build_base_request(&request::LogTail, None)
            .map(|req| self.client.request(req).from_err())
            .into_future()
            .flatten()
            .map(|res| IpfsClient::process_stream_response(res, LineDecoder))
            .flatten_stream();

        Box::new(res)
    }

    /// List the contents of an Ipfs multihash.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.ls(None);
    /// let req = client.ls(Some("/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY"));
    /// # }
    /// ```
    ///
    #[inline]
    pub fn ls(&self, path: Option<&str>) -> AsyncResponse<response::LsResponse> {
        self.request(&request::Ls { path }, None)
    }

    // TODO /mount

    /// Publish an IPFS path to IPNS.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.name_publish(
    ///     "/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY",
    ///     false,
    ///     Some("12h"),
    ///     None,
    ///     None);
    /// # }
    /// ```
    ///
    pub fn name_publish(
        &self,
        path: &str,
        resolve: bool,
        lifetime: Option<&str>,
        ttl: Option<&str>,
        key: Option<&str>,
    ) -> AsyncResponse<response::NamePublishResponse> {
        self.request(
            &request::NamePublish {
                path,
                resolve,
                lifetime,
                ttl,
                key,
            },
            None,
        )
    }

    /// Resolve an IPNS name.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.name_resolve(
    ///     Some("/ipns/ipfs.io"),
    ///     true,
    ///     false);
    /// # }
    /// ```
    ///
    pub fn name_resolve(
        &self,
        name: Option<&str>,
        recursive: bool,
        nocache: bool,
    ) -> AsyncResponse<response::NameResolveResponse> {
        self.request(
            &request::NameResolve {
                name,
                recursive,
                nocache,
            },
            None,
        )
    }

    /// Output the raw bytes of an Ipfs object.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.object_data("/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY");
    /// # }
    /// ```
    ///
    #[inline]
    pub fn object_data(&self, key: &str) -> AsyncStreamResponse<Chunk> {
        self.request_stream_bytes(&request::ObjectData { key }, None)
    }

    /// Returns the diff of two Ipfs objects.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.object_diff(
    ///     "/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY",
    ///     "/ipfs/QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA");
    /// # }
    /// ```
    ///
    #[inline]
    pub fn object_diff(
        &self,
        key0: &str,
        key1: &str,
    ) -> AsyncResponse<response::ObjectDiffResponse> {
        self.request(&request::ObjectDiff { key0, key1 }, None)
    }

    /// Returns the data in an object.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.object_get("/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY");
    /// # }
    /// ```
    ///
    #[inline]
    pub fn object_get(&self, key: &str) -> AsyncResponse<response::ObjectGetResponse> {
        self.request(&request::ObjectGet { key }, None)
    }

    /// Returns the links that an object points to.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.object_links("/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY");
    /// # }
    /// ```
    ///
    #[inline]
    pub fn object_links(&self, key: &str) -> AsyncResponse<response::ObjectLinksResponse> {
        self.request(&request::ObjectLinks { key }, None)
    }

    /// Create a new object.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::{IpfsClient, ObjectTemplate};
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.object_new(None);
    /// let req = client.object_new(Some(ObjectTemplate::UnixFsDir));
    /// # }
    /// ```
    ///
    #[inline]
    pub fn object_new(
        &self,
        template: Option<request::ObjectTemplate>,
    ) -> AsyncResponse<response::ObjectNewResponse> {
        self.request(&request::ObjectNew { template }, None)
    }

    // TODO /object/patch/add-link

    // TODO /object/patch/append-data

    // TODO /object/patch/rm-link

    // TODO /object/patch/set-data

    // TODO /object/put

    /// Returns the stats for an object.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.object_stat("/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY");
    /// # }
    /// ```
    ///
    #[inline]
    pub fn object_stat(&self, key: &str) -> AsyncResponse<response::ObjectStatResponse> {
        self.request(&request::ObjectStat { key }, None)
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
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.pin_add("QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ", true);
    /// # }
    /// ```
    #[inline]
    pub fn pin_add(&self, key: &str, recursive: bool) -> AsyncResponse<response::PinAddResponse> {
        self.request(
            &request::PinAdd {
                key,
                recursive: Some(recursive),
                progress: false,
            },
            None,
        )
    }

    /// Returns a list of pinned objects in local storage.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.pin_ls(None, None);
    /// let req = client.pin_ls(
    ///     Some("/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY"),
    ///     None);
    /// let req = client.pin_ls(None, Some("direct"));
    /// # }
    /// ```
    ///
    #[inline]
    pub fn pin_ls(
        &self,
        key: Option<&str>,
        typ: Option<&str>,
    ) -> AsyncResponse<response::PinLsResponse> {
        self.request(&request::PinLs { key, typ }, None)
    }

    /// Removes a pinned object from local storage.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.pin_rm(
    ///     "/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY",
    ///     false);
    /// let req = client.pin_rm(
    ///     "/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY",
    ///     true);
    /// # }
    /// ```
    ///
    #[inline]
    pub fn pin_rm(&self, key: &str, recursive: bool) -> AsyncResponse<response::PinRmResponse> {
        self.request(&request::PinRm { key, recursive }, None)
    }

    // TODO /pin/update

    // TODO /pin/verify

    /// Pings a peer.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.ping("QmSoLV4Bbm51jM9C4gDYZQ9Cy3U6aXMJDAbzgu2fzaDs64", None);
    /// let req = client.ping("QmSoLV4Bbm51jM9C4gDYZQ9Cy3U6aXMJDAbzgu2fzaDs64", Some(15));
    /// # }
    /// ```
    ///
    #[inline]
    pub fn ping(
        &self,
        peer: &str,
        count: Option<i32>,
    ) -> AsyncStreamResponse<response::PingResponse> {
        self.request_stream_json(&request::Ping { peer, count }, None)
    }

    /// List subscribed pubsub topics.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.pubsub_ls();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn pubsub_ls(&self) -> AsyncResponse<response::PubsubLsResponse> {
        self.request(&request::PubsubLs, None)
    }

    /// List peers that are being published to.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.pubsub_peers(None);
    /// let req = client.pubsub_peers(Some("feed"));
    /// # }
    /// ```
    ///
    #[inline]
    pub fn pubsub_peers(
        &self,
        topic: Option<&str>,
    ) -> AsyncResponse<response::PubsubPeersResponse> {
        self.request(&request::PubsubPeers { topic }, None)
    }

    /// Publish a message to a topic.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.pubsub_pub("feed", "Hello World!");
    /// # }
    /// ```
    ///
    #[inline]
    pub fn pubsub_pub(
        &self,
        topic: &str,
        payload: &str,
    ) -> AsyncResponse<response::PubsubPubResponse> {
        self.request_empty(&request::PubsubPub { topic, payload }, None)
    }

    /// Subscribes to a pubsub topic.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.pubsub_sub("feed", false);
    /// let req = client.pubsub_sub("feed", true);
    /// # }
    /// ```
    ///
    #[inline]
    pub fn pubsub_sub(
        &self,
        topic: &str,
        discover: bool,
    ) -> AsyncStreamResponse<response::PubsubSubResponse> {
        self.request_stream_json(&request::PubsubSub { topic, discover }, None)
    }

    /// Gets a list of local references.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.refs_local();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn refs_local(&self) -> AsyncStreamResponse<response::RefsLocalResponse> {
        self.request_stream_json(&request::RefsLocal, None)
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
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.shutdown();
    /// # }
    /// ```
    ///
    pub fn shutdown(&self) -> AsyncResponse<response::ShutdownResponse> {
        self.request_empty(&request::Shutdown, None)
    }

    /// Returns bitswap stats.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.stats_bitswap();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn stats_bitswap(&self) -> AsyncResponse<response::StatsBitswapResponse> {
        self.request(&request::StatsBitswap, None)
    }

    /// Returns bandwidth stats.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.stats_bw();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn stats_bw(&self) -> AsyncResponse<response::StatsBwResponse> {
        self.request(&request::StatsBw, None)
    }

    /// Returns repo stats.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.stats_repo();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn stats_repo(&self) -> AsyncResponse<response::StatsRepoResponse> {
        self.request(&request::StatsRepo, None)
    }

    // TODO /swarm/addrs/listen

    /// Return a list of local addresses.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.swarm_addrs_local();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn swarm_addrs_local(&self) -> AsyncResponse<response::SwarmAddrsLocalResponse> {
        self.request(&request::SwarmAddrsLocal, None)
    }

    // TODO /swarm/connect

    // TODO /swarm/disconnect

    // TODO /swarm/filters/add

    // TODO /swarm/filters/rm

    /// Return a list of peers with open connections.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.swarm_peers();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn swarm_peers(&self) -> AsyncResponse<response::SwarmPeersResponse> {
        self.request(&request::SwarmPeers, None)
    }

    /// Add a tar file to Ipfs.
    ///
    /// Note: `data` should already be a tar file. If it isn't the Api will return
    /// an error.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    /// use std::fs::File;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let tar = File::open("/path/to/file.tar").unwrap();
    /// let req = client.tar_add(tar);
    /// # }
    /// ```
    ///
    #[inline]
    pub fn tar_add<R>(&self, data: R) -> AsyncResponse<response::TarAddResponse>
    where
        R: 'static + Read + Send,
    {
        let mut form = multipart::Form::default();

        form.add_reader("file", data);

        self.request(&request::TarAdd, Some(form))
    }

    /// Export a tar file from Ipfs.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.tar_cat("/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY");
    /// # }
    /// ```
    ///
    #[inline]
    pub fn tar_cat(&self, path: &str) -> AsyncStreamResponse<Chunk> {
        self.request_stream_bytes(&request::TarCat { path }, None)
    }

    /// Returns information about the Ipfs server version.
    ///
    /// ```no_run
    /// # extern crate ipfs_api;
    /// #
    /// use ipfs_api::IpfsClient;
    ///
    /// # fn main() {
    /// let client = IpfsClient::default();
    /// let req = client.version();
    /// # }
    /// ```
    ///
    #[inline]
    pub fn version(&self) -> AsyncResponse<response::VersionResponse> {
        self.request(&request::Version, None)
    }
}
