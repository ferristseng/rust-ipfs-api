use futures::Stream;
use futures::future::Future;
use request::{self, ApiRequest};
use reqwest::{self, multipart, Method, StatusCode, Url};
use reqwest::unstable::async::{self, Client, ClientBuilder};
use response::{self, Error};
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::Read;
use tokio_core::reactor::Handle;


/// A future response returned by the reqwest HTTP client.
///
type AsyncResponse<T> = Box<Future<Item = T, Error = Error>>;


/// A result returned by a public facing call to the Ipfs api.
///
type ApiResult<T> = Result<AsyncResponse<T>, Error>;


/// Asynchronous Ipfs client.
///
pub struct IpfsClient {
    base: Url,
    client: Client,
}

impl IpfsClient {
    /// Creates a new `IpfsClient`.
    ///
    #[inline]
    pub fn new(
        handle: &Handle,
        host: &str,
        port: u16,
    ) -> Result<IpfsClient, Box<::std::error::Error>> {
        let base_path = IpfsClient::build_base_path(host, port)?;

        Ok(IpfsClient {
            base: base_path,
            client: ClientBuilder::new().build(handle)?,
        })
    }

    /// Builds the base url path for the Ipfs api.
    ///
    fn build_base_path(host: &str, port: u16) -> Result<Url, reqwest::UrlError> {
        format!("http://{}:{}/api/v0", host, port).parse()
    }

    /// Builds the url for an api call.
    ///
    fn build_url<Req>(&self, req: &Req) -> Result<Url, Error>
    where
        Req: ApiRequest + Serialize,
    {
        let uri = format!(
            "{}{}?{}",
            self.base,
            Req::path(),
            ::serde_urlencoded::to_string(req)?
        );

        uri.parse().map_err(From::from)
    }

    /// Processes a response, returning an error or a deserialized json response.
    ///
    fn process_response<Res>(status: StatusCode, chunk: async::Chunk) -> Result<Res, Error>
    where
        for<'de> Res: 'static + Deserialize<'de>,
    {
        match status {
            StatusCode::Ok => serde_json::from_slice(&chunk).map_err(From::from),
            _ => {
                // For error responses, the error can either be a json error,
                // or can just be a string message.
                //
                match serde_json::from_slice(&chunk) {
                    Ok(e) => Err(Error::Api(e)),
                    Err(_) => Err(Error::Uncategorized(String::from_utf8(chunk.to_vec())?)),
                }
            }
        }
    }

    /// Sends a request and returns the raw response.
    ///
    /// Methods prefixed with `send_` work on a raw reqwest `RequestBuilder`
    /// instance.
    ///
    fn send_request(
        &self,
        mut req: async::RequestBuilder,
    ) -> ApiResult<(StatusCode, async::Chunk)> {
        let res = req.send()
            .and_then(|res| {
                let status = res.status();

                res.into_body().concat2().map(move |chunk| (status, chunk))
            })
            .from_err();

        Ok(Box::new(res))
    }

    /// Sends a request and deserializes the response into Json.
    ///
    /// Methods prefixed with `send_` work on a raw reqwest `RequestBuilder`
    /// instance.
    ///
    fn send_request_json<Res>(&self, req: async::RequestBuilder) -> ApiResult<Res>
    where
        for<'de> Res: 'static + Deserialize<'de>,
    {
        let res = self.send_request(req)?.and_then(move |(status, chunk)| {
            IpfsClient::process_response(status, chunk)
        });

        Ok(Box::new(res))
    }

    /// Generates a request, and returns the unprocessed response future.
    ///
    fn request_raw<Req>(&self, req: &Req) -> ApiResult<(StatusCode, async::Chunk)>
    where
        Req: ApiRequest + Serialize,
    {
        let url = self.build_url(req)?;
        let req = self.client.request(Method::Get, url);

        self.send_request(req)
    }

    /// Generic method for making a request to the Ipfs server, and getting
    /// a deserializable response.
    ///
    fn request<Req, Res>(&self, req: &Req) -> ApiResult<Res>
    where
        Req: ApiRequest + Serialize,
        for<'de> Res: 'static + Deserialize<'de>,
    {
        let url = self.build_url(req)?;
        let req = self.client.request(Method::Get, url);

        self.send_request_json(req)
    }

    /// Generic method for making a request to the Ipfs server, and getting
    /// a deserializable response.
    ///
    fn request_with_body<Req, Res, R>(&self, data: R, req: &Req) -> ApiResult<Res>
    where
        Req: ApiRequest + Serialize,
        for<'de> Res: 'static + Deserialize<'de>,
        R: 'static + Read + Send,
    {
        let url = self.build_url(req)?;
        let form = multipart::Form::new().part("file", multipart::Part::reader(data));
        let req = self.client.request(Method::Get, url);

        self.send_request_json(req)
    }
}

impl IpfsClient {
    /// Add file to Ipfs.
    ///
    #[inline]
    pub fn add<R>(&self, data: R) -> ApiResult<response::AddResponse>
    where
        R: 'static + Read + Send,
    {
        self.request_with_body(data, &request::Add)
    }

    /// Add default peers to the bootstrap list.
    ///
    pub fn bootstrap_add_default(&self) -> ApiResult<response::BootstrapAddDefaultResponse> {
        self.request(&request::BootstrapAddDefault)
    }

    /// Lists peers in bootstrap list.
    ///
    pub fn bootstrap_list(&self) -> ApiResult<response::BootstrapListResponse> {
        self.request(&request::BootstrapList)
    }

    /// Removes all peers in bootstrap list.
    ///
    pub fn bootstrap_rm_all(&self) -> ApiResult<response::BootstrapRmAllResponse> {
        self.request(&request::BootstrapRmAll)
    }

    /// List available commands that the server accepts.
    ///
    #[inline]
    pub fn commands(&self) -> ApiResult<response::CommandsResponse> {
        self.request(&request::Commands)
    }

    /// Opens the config file for editing (on the server).
    ///
    pub fn config_edit(&self) -> ApiResult<response::ConfigEditResponse> {
        self.request(&request::ConfigEdit)
    }

    /// Show the current config of the server.
    ///
    /// Returns an unparsed json string, due to an unclear spec.
    ///
    pub fn config_show(&self) -> ApiResult<response::ConfigShowResponse> {
        let req = self.request_raw(&request::ConfigShow)?.and_then(
            |(_, chunk)| {
                String::from_utf8(chunk.to_vec()).map_err(From::from)
            },
        );

        Ok(Box::new(req))
    }

    /// List the contents of an Ipfs multihash.
    ///
    #[inline]
    pub fn ls(&self, path: Option<&str>) -> ApiResult<response::LsResponse> {
        self.request(&request::Ls { path })
    }

    /// Returns the diff of two Ipfs objects.
    ///
    pub fn object_diff(&self, key0: &str, key1: &str) -> ApiResult<response::ObjectDiffResponse> {
        self.request(&request::ObjectDiff { key0, key1 })
    }

    /// Returns the data in an object.
    ///
    pub fn object_get(&self, key: &str) -> ApiResult<response::ObjectGetResponse> {
        self.request(&request::ObjectGet { key })
    }

    /// Returns the links that an object points to.
    ///
    pub fn object_links(&self, key: &str) -> ApiResult<response::ObjectLinksResponse> {
        self.request(&request::ObjectLinks { key })
    }

    /// Returns the stats for an object.
    ///
    pub fn object_stat(&self, key: &str) -> ApiResult<response::ObjectStatResponse> {
        self.request(&request::ObjectStat { key })
    }

    /// Returns a list of pinned objects in local storage.
    ///
    pub fn pin_ls(
        &self,
        key: Option<&str>,
        typ: Option<&str>,
    ) -> ApiResult<response::PinLsResponse> {
        self.request(&request::PinLs { key, typ })
    }

    /// Removes a pinned object from local storage.
    ///
    pub fn pin_rm(&self, key: &str, recursive: Option<bool>) -> ApiResult<response::PinRmResponse> {
        self.request(&request::PinRm { key, recursive })
    }

    /// List subscribed pubsub topics.
    ///
    pub fn pubsub_ls(&self) -> ApiResult<response::PubsubLsResponse> {
        self.request(&request::PubsubLs)
    }

    /// List peers that are being published to.
    ///
    pub fn pubsub_peers(&self, topic: Option<&str>) -> ApiResult<response::PubsubPeersResponse> {
        self.request(&request::PubsubPeers { topic })
    }

    /// Returns bitswap stats.
    ///
    pub fn stats_bitswap(&self) -> ApiResult<response::StatsBitswapResponse> {
        self.request(&request::StatsBitswap)
    }

    /// Returns bandwidth stats.
    ///
    pub fn stats_bw(&self) -> ApiResult<response::StatsBwResponse> {
        self.request(&request::StatsBw)
    }

    /// Returns repo stats.
    ///
    pub fn stats_repo(&self) -> ApiResult<response::StatsRepoResponse> {
        self.request(&request::StatsRepo)
    }

    /// Return a list of local addresses.
    ///
    pub fn swarm_addrs_local(&self) -> ApiResult<response::SwarmAddrsLocalResponse> {
        self.request(&request::SwarmAddrsLocal)
    }

    /// Return a list of peers with open connections.
    ///
    pub fn swarm_peers(&self) -> ApiResult<response::SwarmPeersResponse> {
        self.request(&request::SwarmPeers)
    }

    /// Returns information about the Ipfs server version.
    ///
    #[inline]
    pub fn version(&self) -> ApiResult<response::VersionResponse> {
        self.request(&request::Version)
    }
}
