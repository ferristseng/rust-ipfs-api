use futures::future::Future;
use request::{self, ApiRequest};
use reqwest::{self, multipart, Method, Url};
use reqwest::unstable::async::{Client, ClientBuilder};
use response::{self, Error};
use serde::Deserialize;
use std::io::Read;
use tokio_core::reactor::Handle;


/// A future response returned by the reqwest HTTP client.
///
type AsyncResponse<T> = Box<Future<Item = T, Error = Error>>;


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
    fn build_url<Req>(&self, req: &Req) -> Result<Url, reqwest::UrlError>
    where
        Req: ApiRequest,
    {
        let uri = format!("{}{}?", self.base, Req::path()).parse()?;

        Ok(uri)
    }

    /// Generic method for making a request to the Ipfs server, and getting
    /// a deserializable response.
    ///
    fn request<Req, Res>(&self, req: &Req) -> Result<AsyncResponse<Res>, Error>
    where
        Req: ApiRequest,
        for<'de> Res: 'static + Deserialize<'de>,
    {
        let url = self.build_url(req)?;
        let mut req = self.client.request(Method::Get, url);
        let res = req.send().and_then(move |mut res| res.json()).from_err();

        Ok(Box::new(res))
    }

    /// Generic method for making a request to the Ipfs server, and getting
    /// a deserializable response.
    ///
    fn request_with_body<Req, Res, R>(
        &self,
        data: R,
        req: &Req,
    ) -> Result<AsyncResponse<Res>, Error>
    where
        Req: ApiRequest,
        for<'de> Res: 'static + Deserialize<'de>,
        R: 'static + Read + Send,
    {
        let url = self.build_url(req)?;
        let form = multipart::Form::new().part("file", multipart::Part::reader(data));
        let mut req = self.client.request(Method::Get, url); //.form(&form);
        let res = req.send().and_then(move |mut res| res.json()).from_err();

        Ok(Box::new(res))
    }
}

impl IpfsClient {
    /// Add file to Ipfs.
    ///
    #[inline]
    pub fn add<R>(&self, data: R) -> Result<AsyncResponse<response::AddResponse>, Error>
    where
        R: 'static + Read + Send,
    {
        self.request_with_body(data, &request::Add)
    }

    /// List available commands that the server accepts.
    ///
    #[inline]
    pub fn commands(&self) -> Result<AsyncResponse<response::CommandsResponse>, Error> {
        self.request(&request::Commands)
    }

    /// List the contents of an Ipfs multihash.
    ///
    #[inline]
    pub fn ls(&self, path: Option<&str>) -> Result<AsyncResponse<response::LsResponse>, Error> {
        self.request(&request::LsRequest(path))
    }

    /// Returns information about the Ipfs server version.
    ///
    #[inline]
    pub fn version(&self) -> Result<AsyncResponse<response::VersionResponse>, Error> {
        self.request(&request::Version)
    }
}
