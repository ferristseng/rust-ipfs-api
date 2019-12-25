// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
use crate::{
    header::TRAILER,
    read::{JsonLineDecoder, LineDecoder, StreamReader},
    request::{self, ApiRequest},
    response::{self, Error},
    AsyncStreamResponse, Client, Request, Response,
};
#[cfg(feature = "actix")]
use actix_multipart::client::multipart;
use bytes::Bytes;
use futures::{future, FutureExt, Stream, TryFutureExt, TryStreamExt};
use http::{
    uri::{InvalidUri, Uri},
    StatusCode,
};
#[cfg(feature = "hyper")]
use hyper::{body, client::Builder};
#[cfg(feature = "hyper")]
use hyper_multipart::client::multipart;
#[cfg(feature = "hyper")]
use hyper_tls::HttpsConnector;
use multiaddr::{AddrComponent, ToMultiaddr};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fs,
    io::{Cursor, Read},
    net::{IpAddr, SocketAddr},
    path::{Path, PathBuf},
};
use tokio_util::codec::{Decoder, FramedRead};

const FILE_DESCRIPTOR_LIMIT: usize = 127;

/// Asynchronous Ipfs client.
///
#[derive(Clone)]
pub struct IpfsClient {
    base: Uri,
    client: Client,
}

impl Default for IpfsClient {
    /// Creates an `IpfsClient` connected to the endpoint specified in ~/.ipfs/api.
    /// If not found, tries to connect to `localhost:5001`.
    ///
    fn default() -> IpfsClient {
        dirs::home_dir()
            .map(|home_dir| home_dir.join(".ipfs").join("api"))
            .and_then(|multiaddr_path| fs::read_to_string(&multiaddr_path).ok())
            .and_then(|multiaddr_str| multiaddr_str.to_multiaddr().ok())
            .and_then(|multiaddr| {
                let mut addr: Option<IpAddr> = None;
                let mut port: Option<u16> = None;
                for addr_component in multiaddr.iter() {
                    match addr_component {
                        AddrComponent::IP4(v4addr) => addr = Some(v4addr.into()),
                        AddrComponent::IP6(v6addr) => addr = Some(v6addr.into()),
                        AddrComponent::TCP(tcpport) => port = Some(tcpport),
                        _ => {
                            return None;
                        }
                    }
                }
                if let (Some(addr), Some(port)) = (addr, port) {
                    Some(SocketAddr::new(addr, port))
                } else {
                    None
                }
            })
            .map(IpfsClient::from)
            .unwrap_or_else(|| IpfsClient::new("localhost", 5001).unwrap())
    }
}

impl From<SocketAddr> for IpfsClient {
    fn from(socket_addr: SocketAddr) -> Self {
        IpfsClient::new(&socket_addr.ip().to_string(), socket_addr.port()).unwrap()
    }
}

impl IpfsClient {
    /// Creates a new `IpfsClient`.
    ///
    #[inline]
    pub fn new(host: &str, port: u16) -> Result<IpfsClient, InvalidUri> {
        Self::new_from_uri(format!("http://{}:{}", host, port).as_str())
    }

    /// Creates a new `IpfsClient` for any given URI.
    ///
    #[inline]
    pub fn new_from_uri(uri: &str) -> Result<IpfsClient, InvalidUri> {
        let base_path = IpfsClient::build_base_path(uri)?;
        let client = {
            #[cfg(feature = "hyper")]
            {
                Builder::default()
                    .keep_alive(false)
                    .build(HttpsConnector::new())
            }
            #[cfg(feature = "actix")]
            {
                Client::default()
            }
        };

        Ok(IpfsClient {
            base: base_path,
            client,
        })
    }

    /// Builds the base url path for the Ipfs api.
    ///
    fn build_base_path(uri: &str) -> Result<Uri, InvalidUri> {
        format!("{}/api/v0", uri).parse()
    }

    /// Builds the url for an api call.
    ///
    fn build_base_request<Req>(
        &self,
        req: Req,
        form: Option<multipart::Form<'static>>,
    ) -> Result<Request, Error>
    where
        Req: ApiRequest + Serialize,
    {
        let url = format!(
            "{}{}?{}",
            self.base,
            Req::PATH,
            ::serde_urlencoded::to_string(req)?
        );

        #[cfg(feature = "hyper")]
        {
            url.parse::<Uri>().map_err(From::from).and_then(move |url| {
                let builder = http::Request::builder();
                let builder = builder.method(Req::METHOD.clone()).uri(url);

                let req = if let Some(form) = form {
                    form.set_body_convert::<hyper::Body, multipart::Body>(builder)
                } else {
                    builder.body(hyper::Body::empty())
                };

                req.map_err(From::from)
            })
        }
        #[cfg(feature = "actix")]
        {
            let req = if let Some(form) = form {
                self.client
                    .request(Req::METHOD.clone(), url)
                    .content_type(form.content_type())
            } else {
                self.client.request(Req::METHOD.clone(), url)
            };

            Ok(req.timeout(std::time::Duration::from_secs(90)))
        }
    }

    /// Builds an Api error from a response body.
    ///
    #[inline]
    fn process_error_from_body(body: Bytes) -> Error {
        match serde_json::from_slice(&body) {
            Ok(e) => Error::Api(e),
            Err(_) => match String::from_utf8(body.to_vec()) {
                Ok(s) => Error::Uncategorized(s),
                Err(e) => e.into(),
            },
        }
    }

    /// Processes a response that expects a json encoded body, returning an
    /// error or a deserialized json response.
    ///
    fn process_json_response<Res>(status: StatusCode, body: Bytes) -> Result<Res, Error>
    where
        for<'de> Res: 'static + Deserialize<'de>,
    {
        match status {
            StatusCode::OK => serde_json::from_slice(&body).map_err(From::from),
            _ => Err(Self::process_error_from_body(body)),
        }
    }

    /// Processes a response that returns a stream of json deserializable
    /// results.
    ///
    fn process_stream_response<D, Res>(
        res: Response,
        decoder: D,
    ) -> impl Stream<Item = Result<Res, Error>>
    where
        D: Decoder<Item = Res, Error = Error> + Send,
    {
        #[cfg(feature = "hyper")]
        {
            FramedRead::new(StreamReader::new(res.into_body()), decoder)
        }
        #[cfg(feature = "actix")]
        {
            FramedRead::new(StreamReader::new(res), decoder)
        }
    }

    /// Generates a request, and returns the unprocessed response future.
    ///
    async fn request_raw<Req>(
        &self,
        req: Req,
        form: Option<multipart::Form<'static>>,
    ) -> Result<(StatusCode, Bytes), Error>
    where
        Req: ApiRequest + Serialize,
    {
        let req = self.build_base_request(req, form)?;

        #[cfg(feature = "hyper")]
        {
            let res = self.client.request(req).await?;
            let status = res.status();
            let body = body::to_bytes(res.into_body()).await?;

            Ok((status, body))
        }
        #[cfg(feature = "actix")]
        {
            let mut res = req.send().await?;
            let status = res.status();
            let body = res.body().await?;

            Ok((status, body))
        }
    }

    /// Generic method for making a request that expects back a streaming
    /// response.
    ///
    fn request_stream<Req, Res, F>(
        &self,
        req: Req,
        form: Option<multipart::Form<'static>>,
        process: F,
    ) -> AsyncStreamResponse<Res>
    where
        Req: ApiRequest + Serialize,
        F: 'static + Send + Fn(Response) -> AsyncStreamResponse<Res>,
    {
        let request = future::ready(self.build_base_request(req, form));

        let response = {
            #[cfg(feature = "hyper")]
            {
                let client = self.client.clone();

                request
                    .and_then(move |req| client.request(req).err_into())
                    .map_ok(move |res| {
                        match res.status() {
                            StatusCode::OK => process(res),
                            // If the server responded with an error status code, the body
                            // still needs to be read so an error can be built. This block will
                            // read the entire body stream, then immediately return an error.
                            //
                            _ => Box::new(
                                body::to_bytes(res.into_body())
                                    .boxed()
                                    .map(|maybe_body| match maybe_body {
                                        Ok(body) => Err(Self::process_error_from_body(body)),
                                        Err(e) => Err(e.into()),
                                    })
                                    .into_stream(),
                            ),
                        }
                    })
                    .try_flatten_stream()
            }
            #[cfg(feature = "actix")]
            {
                request
                    .and_then(|req| req.send().err_into())
                    .map_ok(move |mut res| {
                        match res.status() {
                            StatusCode::OK => process(res),
                            // If the server responded with an error status code, the body
                            // still needs to be read so an error can be built. This block will
                            // read the entire body stream, then immediately return an error.
                            //
                            _ => Box::new(
                                res.body()
                                    .map(|maybe_body| match maybe_body {
                                        Ok(body) => Err(Self::process_error_from_body(body)),
                                        Err(e) => Err(e.into()),
                                    })
                                    .into_stream(),
                            ),
                        }
                    })
                    .try_flatten_stream()
            }
        };

        Box::new(response)
    }

    /// Generic method for making a request to the Ipfs server, and getting
    /// a deserializable response.
    ///
    async fn request<Req, Res>(
        &self,
        req: Req,
        form: Option<multipart::Form<'static>>,
    ) -> Result<Res, Error>
    where
        Req: ApiRequest + Serialize,
        for<'de> Res: 'static + Deserialize<'de>,
    {
        let (status, chunk) = self.request_raw(req, form).await?;

        IpfsClient::process_json_response(status, chunk)
    }

    /// Generic method for making a request to the Ipfs server, and getting
    /// back a response with no body.
    ///
    async fn request_empty<Req>(
        &self,
        req: Req,
        form: Option<multipart::Form<'static>>,
    ) -> Result<(), Error>
    where
        Req: ApiRequest + Serialize,
    {
        let (status, chunk) = self.request_raw(req, form).await?;

        match status {
            StatusCode::OK => Ok(()),
            _ => Err(Self::process_error_from_body(chunk)),
        }
    }

    /// Generic method for making a request to the Ipfs server, and getting
    /// back a raw String response.
    ///
    async fn request_string<Req>(
        &self,
        req: Req,
        form: Option<multipart::Form<'static>>,
    ) -> Result<String, Error>
    where
        Req: ApiRequest + Serialize,
    {
        let (status, chunk) = self.request_raw(req, form).await?;

        match status {
            StatusCode::OK => String::from_utf8(chunk.to_vec()).map_err(From::from),
            _ => Err(Self::process_error_from_body(chunk)),
        }
    }

    /// Generic method for making a request to the Ipfs server, and getting
    /// back a raw stream of bytes.
    ///
    fn request_stream_bytes<Req>(
        &self,
        req: Req,
        form: Option<multipart::Form<'static>>,
    ) -> AsyncStreamResponse<Bytes>
    where
        Req: ApiRequest + Serialize,
    {
        #[cfg(feature = "hyper")]
        {
            self.request_stream(req, form, |res| Box::new(res.into_body().err_into()))
        }
        #[cfg(feature = "actix")]
        {
            self.request_stream(req, form, |res| Box::new(res.err_into()))
        }
    }

    /// Generic method to return a streaming response of deserialized json
    /// objects delineated by new line separators.
    ///
    fn request_stream_json<Req, Res>(
        &self,
        req: Req,
        form: Option<multipart::Form<'static>>,
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
                    return Box::new(future::err(err).into_stream());
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
    /// use ipfs_api::IpfsClient;
    /// use std::io::Cursor;
    ///
    /// let client = IpfsClient::default();
    /// let data = Cursor::new("Hello World!");
    /// let res = client.add(data);
    /// ```
    ///
    #[inline]
    pub async fn add<R>(&self, data: R) -> Result<response::AddResponse, Error>
    where
        R: 'static + Read + Send + Sync,
    {
        let mut form = multipart::Form::default();

        form.add_reader("path", data);

        self.request(request::Add, Some(form)).await
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
    #[inline]
    pub async fn add_path<P>(&self, path: P) -> Result<Vec<response::AddResponse>, Error>
    where
        P: AsRef<Path>,
    {
        let prefix = path.as_ref().parent();
        let mut paths_to_add: Vec<(PathBuf, u64)> = vec![];

        for path in walkdir::WalkDir::new(path.as_ref()) {
            match path {
                Ok(entry) if entry.file_type().is_file() => {
                    if entry.file_type().is_file() {
                        let file_size = entry
                            .metadata()
                            .map(|metadata| metadata.len())
                            .map_err(|e| Error::Io(e.into()))?;

                        paths_to_add.push((entry.path().to_path_buf(), file_size));
                    }
                }
                Ok(_) => (),
                Err(err) => return Err(Error::Io(err.into())),
            }
        }

        paths_to_add.sort_unstable_by(|(_, a), (_, b)| a.cmp(b).reverse());

        let mut it = 0;
        let mut form = multipart::Form::default();

        for (path, file_size) in paths_to_add {
            let mut file = fs::File::open(&path)?;
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
                let _ = file.read_to_end(&mut buf)?;

                form.add_reader_file("path", Cursor::new(buf), file_name);
            }
        }

        self.request_stream_json(request::Add, Some(form))
            .try_collect()
            .await
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
    #[inline]
    pub async fn bitswap_ledger(
        &self,
        peer: &str,
    ) -> Result<response::BitswapLedgerResponse, Error> {
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
    #[inline]
    pub async fn bitswap_reprovide(&self) -> Result<response::BitswapReprovideResponse, Error> {
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
    #[inline]
    pub async fn bitswap_stat(&self) -> Result<response::BitswapStatResponse, Error> {
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
    #[inline]
    pub async fn bitswap_unwant(
        &self,
        key: &str,
    ) -> Result<response::BitswapUnwantResponse, Error> {
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
    #[inline]
    pub async fn bitswap_wantlist(
        &self,
        peer: Option<&str>,
    ) -> Result<response::BitswapWantlistResponse, Error> {
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
    #[inline]
    pub fn block_get(&self, hash: &str) -> impl Stream<Item = Result<Bytes, Error>> {
        self.request_stream_bytes(request::BlockGet { hash }, None)
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
    #[inline]
    pub async fn block_put<R>(&self, data: R) -> Result<response::BlockPutResponse, Error>
    where
        R: 'static + Read + Send + Sync,
    {
        let mut form = multipart::Form::default();

        form.add_reader("data", data);

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
    #[inline]
    pub async fn block_rm(&self, hash: &str) -> Result<response::BlockRmResponse, Error> {
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
    #[inline]
    pub async fn block_stat(&self, hash: &str) -> Result<response::BlockStatResponse, Error> {
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
    #[inline]
    pub async fn bootstrap_add_default(
        &self,
    ) -> Result<response::BootstrapAddDefaultResponse, Error> {
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
    #[inline]
    pub async fn bootstrap_list(&self) -> Result<response::BootstrapListResponse, Error> {
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
    #[inline]
    pub async fn bootstrap_rm_all(&self) -> Result<response::BootstrapRmAllResponse, Error> {
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
    #[inline]
    pub fn cat(&self, path: &str) -> AsyncStreamResponse<Bytes> {
        self.request_stream_bytes(request::Cat { path }, None)
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
    #[inline]
    pub async fn commands(&self) -> Result<response::CommandsResponse, Error> {
        self.request(request::Commands, None).await
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
    #[inline]
    pub async fn config_edit(&self) -> Result<response::ConfigEditResponse, Error> {
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
    #[inline]
    pub async fn config_replace<R>(&self, data: R) -> Result<response::ConfigReplaceResponse, Error>
    where
        R: 'static + Read + Send + Sync,
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
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.config_show();
    /// ```
    ///
    #[inline]
    pub async fn config_show(&self) -> Result<response::ConfigShowResponse, Error> {
        self.request_string(request::ConfigShow, None).await
    }

    /// Returns information about a dag node in Ipfs.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.dag_get("QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA");
    /// ```
    ///
    #[inline]
    pub async fn dag_get(&self, path: &str) -> Result<response::DagGetResponse, Error> {
        self.request(request::DagGet { path }, None).await
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
    /// use futures::TryStreamExt;
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let peer = "QmSoLPppuBtQSGwKDZT2M73ULpjvfd3aZ6ha4oFGL1KrGM";
    /// let res = client.dht_findpeer(peer).try_collect::<Vec<_>>();
    /// ```
    ///
    #[inline]
    pub fn dht_findpeer(&self, peer: &str) -> AsyncStreamResponse<response::DhtFindPeerResponse> {
        self.request_stream_json(request::DhtFindPeer { peer }, None)
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
    #[inline]
    pub fn dht_findprovs(&self, key: &str) -> AsyncStreamResponse<response::DhtFindProvsResponse> {
        self.request_stream_json(request::DhtFindProvs { key }, None)
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
    #[inline]
    pub fn dht_get(&self, key: &str) -> AsyncStreamResponse<response::DhtGetResponse> {
        self.request_stream_json(request::DhtGet { key }, None)
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
    #[inline]
    pub fn dht_provide(&self, key: &str) -> AsyncStreamResponse<response::DhtProvideResponse> {
        self.request_stream_json(request::DhtProvide { key }, None)
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
    #[inline]
    pub fn dht_put(&self, key: &str, value: &str) -> AsyncStreamResponse<response::DhtPutResponse> {
        self.request_stream_json(request::DhtPut { key, value }, None)
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
    #[inline]
    pub fn dht_query(&self, peer: &str) -> AsyncStreamResponse<response::DhtQueryResponse> {
        self.request_stream_json(request::DhtQuery { peer }, None)
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
    #[inline]
    pub async fn diag_cmds_clear(&self) -> Result<response::DiagCmdsClearResponse, Error> {
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
    #[inline]
    pub async fn diag_cmds_set_time(
        &self,
        time: &str,
    ) -> Result<response::DiagCmdsSetTimeResponse, Error> {
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
    #[inline]
    pub async fn diag_sys(&self) -> Result<response::DiagSysResponse, Error> {
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
    #[inline]
    pub async fn dns(&self, link: &str, recursive: bool) -> Result<response::DnsResponse, Error> {
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
    #[inline]
    pub async fn file_ls(&self, path: &str) -> Result<response::FileLsResponse, Error> {
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
    #[inline]
    pub async fn files_cp(
        &self,
        path: &str,
        dest: &str,
    ) -> Result<response::FilesCpResponse, Error> {
        self.request_empty(request::FilesCp { path, dest }, None)
            .await
    }

    /// Flush a path's data to disk.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.files_flush(None);
    /// let res = client.files_flush(Some("/tmp"));
    /// ```
    ///
    #[inline]
    pub async fn files_flush(
        &self,
        path: Option<&str>,
    ) -> Result<response::FilesFlushResponse, Error> {
        self.request_empty(request::FilesFlush { path }, None).await
    }

    /// List directories in MFS.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.files_ls(None);
    /// let res = client.files_ls(Some("/tmp"));
    /// ```
    ///
    #[inline]
    pub async fn files_ls(&self, path: Option<&str>) -> Result<response::FilesLsResponse, Error> {
        self.request(request::FilesLs { path }, None).await
    }

    /// Make directories in MFS.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.files_mkdir("/test", false);
    /// let res = client.files_mkdir("/test/nested/dir", true);
    /// ```
    ///
    #[inline]
    pub async fn files_mkdir(
        &self,
        path: &str,
        parents: bool,
    ) -> Result<response::FilesMkdirResponse, Error> {
        self.request_empty(request::FilesMkdir { path, parents }, None)
            .await
    }

    /// Copy files into MFS.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.files_mv("/test/tmp.json", "/test/file.json");
    /// ```
    ///
    #[inline]
    pub async fn files_mv(
        &self,
        path: &str,
        dest: &str,
    ) -> Result<response::FilesMvResponse, Error> {
        self.request_empty(request::FilesMv { path, dest }, None)
            .await
    }

    /// Read a file in MFS.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.files_read("/test/file.json");
    /// ```
    ///
    #[inline]
    pub fn files_read(&self, path: &str) -> AsyncStreamResponse<Bytes> {
        self.request_stream_bytes(request::FilesRead { path }, None)
    }

    /// Remove a file in MFS.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.files_rm("/test/dir", true);
    /// let res = client.files_rm("/test/file.json", false);
    /// ```
    ///
    #[inline]
    pub async fn files_rm(
        &self,
        path: &str,
        recursive: bool,
    ) -> Result<response::FilesRmResponse, Error> {
        self.request_empty(request::FilesRm { path, recursive }, None)
            .await
    }

    /// Display a file's status in MDFS.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.files_stat("/test/file.json");
    /// ```
    ///
    #[inline]
    pub async fn files_stat(&self, path: &str) -> Result<response::FilesStatResponse, Error> {
        self.request(request::FilesStat { path }, None).await
    }

    /// Write to a mutable file in the filesystem.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    /// use std::fs::File;
    ///
    /// let client = IpfsClient::default();
    /// let file = File::open("test.json").unwrap();
    /// let res = client.files_write("/test/file.json", true, true, file);
    /// ```
    ///
    #[inline]
    pub async fn files_write<R>(
        &self,
        path: &str,
        create: bool,
        truncate: bool,
        data: R,
    ) -> Result<response::FilesWriteResponse, Error>
    where
        R: 'static + Read + Send + Sync,
    {
        let mut form = multipart::Form::default();

        form.add_reader("data", data);

        self.request_empty(
            request::FilesWrite {
                path,
                create,
                truncate,
            },
            Some(form),
        )
        .await
    }

    /// List blocks that are both in the filestore and standard block storage.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.filestore_dups();
    /// ```
    ///
    #[inline]
    pub fn filestore_dups(&self) -> AsyncStreamResponse<response::FilestoreDupsResponse> {
        self.request_stream_json(request::FilestoreDups, None)
    }

    /// List objects in filestore.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.filestore_ls(
    ///     Some("QmYPP3BovR2m8UqCZxFbdXSit6SKgExxDkFAPLqiGsap4X")
    /// );
    /// ```
    ///
    #[inline]
    pub fn filestore_ls(
        &self,
        cid: Option<&str>,
    ) -> AsyncStreamResponse<response::FilestoreLsResponse> {
        self.request_stream_json(request::FilestoreLs { cid }, None)
    }

    /// Verify objects in filestore.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.filestore_verify(None);
    /// ```
    ///
    #[inline]
    pub fn filestore_verify(
        &self,
        cid: Option<&str>,
    ) -> AsyncStreamResponse<response::FilestoreVerifyResponse> {
        self.request_stream_json(request::FilestoreVerify { cid }, None)
    }

    /// Download Ipfs object.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.get("/test/file.json");
    /// ```
    ///
    #[inline]
    pub fn get(&self, path: &str) -> AsyncStreamResponse<Bytes> {
        self.request_stream_bytes(request::Get { path }, None)
    }

    /// Returns information about a peer.
    ///
    /// If `peer` is `None`, returns information about you.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.id(None);
    /// let res = client.id(Some("QmSoLPppuBtQSGwKDZT2M73ULpjvfd3aZ6ha4oFGL1KrGM"));
    /// ```
    ///
    #[inline]
    pub async fn id(&self, peer: Option<&str>) -> Result<response::IdResponse, Error> {
        self.request(request::Id { peer }, None).await
    }

    /// Create a new keypair.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsClient, KeyType};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.key_gen("test", KeyType::Rsa, 64);
    /// ```
    ///
    #[inline]
    pub async fn key_gen(
        &self,
        name: &str,
        kind: request::KeyType,
        size: i32,
    ) -> Result<response::KeyGenResponse, Error> {
        self.request(request::KeyGen { name, kind, size }, None)
            .await
    }

    /// List all local keypairs.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.key_list();
    /// ```
    ///
    #[inline]
    pub async fn key_list(&self) -> Result<response::KeyListResponse, Error> {
        self.request(request::KeyList, None).await
    }

    /// Rename a keypair.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.key_rename("key_0", "new_name", false);
    /// ```
    ///
    #[inline]
    pub async fn key_rename(
        &self,
        name: &str,
        new: &str,
        force: bool,
    ) -> Result<response::KeyRenameResponse, Error> {
        self.request(request::KeyRename { name, new, force }, None)
            .await
    }

    /// Remove a keypair.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.key_rm("key_0");
    /// ```
    ///
    #[inline]
    pub async fn key_rm(&self, name: &str) -> Result<response::KeyRmResponse, Error> {
        self.request(request::KeyRm { name }, None).await
    }

    /// Change the logging level for a logger.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsClient, Logger, LoggingLevel};
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
    #[inline]
    pub async fn log_level(
        &self,
        logger: request::Logger<'_>,
        level: request::LoggingLevel,
    ) -> Result<response::LogLevelResponse, Error> {
        self.request(request::LogLevel { logger, level }, None)
            .await
    }

    /// List all logging subsystems.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.log_ls();
    /// ```
    ///
    #[inline]
    pub async fn log_ls(&self) -> Result<response::LogLsResponse, Error> {
        self.request(request::LogLs, None).await
    }

    /// Read the event log.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.log_tail();
    /// ```
    ///
    pub fn log_tail(&self) -> AsyncStreamResponse<String> {
        self.request_stream(request::LogTail, None, |res| {
            Box::new(IpfsClient::process_stream_response(res, LineDecoder))
        })
    }

    /// List the contents of an Ipfs multihash.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.ls(None);
    /// let res = client.ls(Some("/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY"));
    /// ```
    ///
    #[inline]
    pub async fn ls(&self, path: Option<&str>) -> Result<response::LsResponse, Error> {
        self.request(request::Ls { path }, None).await
    }

    // TODO /mount

    /// Publish an IPFS path to IPNS.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
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
    pub async fn name_publish(
        &self,
        path: &str,
        resolve: bool,
        lifetime: Option<&str>,
        ttl: Option<&str>,
        key: Option<&str>,
    ) -> Result<response::NamePublishResponse, Error> {
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
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.name_resolve(
    ///     Some("/ipns/ipfs.io"),
    ///     true,
    ///     false
    /// );
    /// ```
    ///
    pub async fn name_resolve(
        &self,
        name: Option<&str>,
        recursive: bool,
        nocache: bool,
    ) -> Result<response::NameResolveResponse, Error> {
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
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.object_data("/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY");
    /// ```
    ///
    #[inline]
    pub fn object_data(&self, key: &str) -> AsyncStreamResponse<Bytes> {
        self.request_stream_bytes(request::ObjectData { key }, None)
    }

    /// Returns the diff of two Ipfs objects.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.object_diff(
    ///     "/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY",
    ///     "/ipfs/QmXdNSQx7nbdRvkjGCEQgVjVtVwsHvV8NmV2a8xzQVwuFA"
    /// );
    /// ```
    ///
    #[inline]
    pub async fn object_diff(
        &self,
        key0: &str,
        key1: &str,
    ) -> Result<response::ObjectDiffResponse, Error> {
        self.request(request::ObjectDiff { key0, key1 }, None).await
    }

    /// Returns the data in an object.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.object_get("/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY");
    /// ```
    ///
    #[inline]
    pub async fn object_get(&self, key: &str) -> Result<response::ObjectGetResponse, Error> {
        self.request(request::ObjectGet { key }, None).await
    }

    /// Returns the links that an object points to.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.object_links("/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY");
    /// ```
    ///
    #[inline]
    pub async fn object_links(&self, key: &str) -> Result<response::ObjectLinksResponse, Error> {
        self.request(request::ObjectLinks { key }, None).await
    }

    /// Create a new object.
    ///
    /// ```no_run
    /// use ipfs_api::{IpfsClient, ObjectTemplate};
    ///
    /// let client = IpfsClient::default();
    /// let res = client.object_new(None);
    /// let res = client.object_new(Some(ObjectTemplate::UnixFsDir));
    /// ```
    ///
    #[inline]
    pub async fn object_new(
        &self,
        template: Option<request::ObjectTemplate>,
    ) -> Result<response::ObjectNewResponse, Error> {
        self.request(request::ObjectNew { template }, None).await
    }

    // TODO /object/patch/add-link

    // TODO /object/patch/append-data

    // TODO /object/patch/rm-link

    // TODO /object/patch/set-data

    // TODO /object/put

    /// Returns the stats for an object.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.object_stat("/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY");
    /// ```
    ///
    #[inline]
    pub async fn object_stat(&self, key: &str) -> Result<response::ObjectStatResponse, Error> {
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
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.pin_add("QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ", true);
    /// ```
    #[inline]
    pub async fn pin_add(
        &self,
        key: &str,
        recursive: bool,
    ) -> Result<response::PinAddResponse, Error> {
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
    /// use ipfs_api::IpfsClient;
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
    #[inline]
    pub async fn pin_ls(
        &self,
        key: Option<&str>,
        typ: Option<&str>,
    ) -> Result<response::PinLsResponse, Error> {
        self.request(request::PinLs { key, typ }, None).await
    }

    /// Removes a pinned object from local storage.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
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
    #[inline]
    pub async fn pin_rm(
        &self,
        key: &str,
        recursive: bool,
    ) -> Result<response::PinRmResponse, Error> {
        self.request(request::PinRm { key, recursive }, None).await
    }

    // TODO /pin/update

    // TODO /pin/verify

    /// Pings a peer.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.ping("QmSoLV4Bbm51jM9C4gDYZQ9Cy3U6aXMJDAbzgu2fzaDs64", None);
    /// let res = client.ping("QmSoLV4Bbm51jM9C4gDYZQ9Cy3U6aXMJDAbzgu2fzaDs64", Some(15));
    /// ```
    ///
    #[inline]
    pub fn ping(
        &self,
        peer: &str,
        count: Option<i32>,
    ) -> AsyncStreamResponse<response::PingResponse> {
        self.request_stream_json(request::Ping { peer, count }, None)
    }

    /// List subscribed pubsub topics.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.pubsub_ls();
    /// ```
    ///
    #[inline]
    pub async fn pubsub_ls(&self) -> Result<response::PubsubLsResponse, Error> {
        self.request(request::PubsubLs, None).await
    }

    /// List peers that are being published to.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.pubsub_peers(None);
    /// let res = client.pubsub_peers(Some("feed"));
    /// ```
    ///
    #[inline]
    pub async fn pubsub_peers(
        &self,
        topic: Option<&str>,
    ) -> Result<response::PubsubPeersResponse, Error> {
        self.request(request::PubsubPeers { topic }, None).await
    }

    /// Publish a message to a topic.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.pubsub_pub("feed", "Hello World!");
    /// ```
    ///
    #[inline]
    pub async fn pubsub_pub(
        &self,
        topic: &str,
        payload: &str,
    ) -> Result<response::PubsubPubResponse, Error> {
        self.request_empty(request::PubsubPub { topic, payload }, None)
            .await
    }

    /// Subscribes to a pubsub topic.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.pubsub_sub("feed", false);
    /// let res = client.pubsub_sub("feed", true);
    /// ```
    ///
    #[inline]
    pub fn pubsub_sub(
        &self,
        topic: &str,
        discover: bool,
    ) -> AsyncStreamResponse<response::PubsubSubResponse> {
        self.request_stream_json(request::PubsubSub { topic, discover }, None)
    }

    /// Gets a list of local references.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.refs_local();
    /// ```
    ///
    #[inline]
    pub fn refs_local(&self) -> AsyncStreamResponse<response::RefsLocalResponse> {
        self.request_stream_json(request::RefsLocal, None)
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
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.shutdown();
    /// ```
    ///
    pub async fn shutdown(&self) -> Result<response::ShutdownResponse, Error> {
        self.request_empty(request::Shutdown, None).await
    }

    /// Returns bitswap stats.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.stats_bitswap();
    /// ```
    ///
    #[inline]
    pub async fn stats_bitswap(&self) -> Result<response::StatsBitswapResponse, Error> {
        self.request(request::StatsBitswap, None).await
    }

    /// Returns bandwidth stats.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.stats_bw();
    /// ```
    ///
    #[inline]
    pub async fn stats_bw(&self) -> Result<response::StatsBwResponse, Error> {
        self.request(request::StatsBw, None).await
    }

    /// Returns repo stats.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.stats_repo();
    /// ```
    ///
    #[inline]
    pub async fn stats_repo(&self) -> Result<response::StatsRepoResponse, Error> {
        self.request(request::StatsRepo, None).await
    }

    // TODO /swarm/addrs/listen

    /// Return a list of local addresses.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.swarm_addrs_local();
    /// ```
    ///
    #[inline]
    pub async fn swarm_addrs_local(&self) -> Result<response::SwarmAddrsLocalResponse, Error> {
        self.request(request::SwarmAddrsLocal, None).await
    }

    // TODO /swarm/connect

    // TODO /swarm/disconnect

    // TODO /swarm/filters/add

    // TODO /swarm/filters/rm

    /// Return a list of peers with open connections.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.swarm_peers();
    /// ```
    ///
    #[inline]
    pub async fn swarm_peers(&self) -> Result<response::SwarmPeersResponse, Error> {
        self.request(request::SwarmPeers, None).await
    }

    /// Add a tar file to Ipfs.
    ///
    /// Note: `data` should already be a tar file. If it isn't the Api will return
    /// an error.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    /// use std::fs::File;
    ///
    /// let client = IpfsClient::default();
    /// let tar = File::open("/path/to/file.tar").unwrap();
    /// let res = client.tar_add(tar);
    /// ```
    ///
    #[inline]
    pub async fn tar_add<R>(&self, data: R) -> Result<response::TarAddResponse, Error>
    where
        R: 'static + Read + Send + Sync,
    {
        let mut form = multipart::Form::default();

        form.add_reader("file", data);

        self.request(request::TarAdd, Some(form)).await
    }

    /// Export a tar file from Ipfs.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.tar_cat("/ipfs/QmVrLsEDn27sScp3k23sgZNefVTjSAL3wpgW1iWPi4MgoY");
    /// ```
    ///
    #[inline]
    pub fn tar_cat(&self, path: &str) -> AsyncStreamResponse<Bytes> {
        self.request_stream_bytes(request::TarCat { path }, None)
    }

    /// Returns information about the Ipfs server version.
    ///
    /// ```no_run
    /// use ipfs_api::IpfsClient;
    ///
    /// let client = IpfsClient::default();
    /// let res = client.version();
    /// ```
    ///
    #[inline]
    pub async fn version(&self) -> Result<response::VersionResponse, Error> {
        self.request(request::Version, None).await
    }
}
