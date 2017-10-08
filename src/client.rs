use futures::Stream;
use futures::future::Future;
use hyper::Uri;
use hyper::client::{Client, HttpConnector};
use hyper::error::UriError;
use request::{self, ApiRequest};
use response::{self, Error};
use serde::Deserialize;
use serde_json;
use tokio_core::reactor::Handle;


pub type AsyncResponse<T> = Box<Future<Item = T, Error = Error>>;


/// Asynchronous Ipfs client.
///
pub struct IpfsClient {
    base: Uri,
    client: Client<HttpConnector>,
}

impl IpfsClient {
    /// Creates a new `IpfsClient`.
    ///
    #[inline]
    pub fn new(handle: &Handle, host: &str, port: u16) -> Result<IpfsClient, UriError> {
        let base_path = IpfsClient::build_base_path(host, port)?;

        Ok(IpfsClient {
            base: base_path,
            client: Client::new(handle),
        })
    }

    /// Builds the base uri path for the Ipfs API.
    ///
    fn build_base_path(host: &str, port: u16) -> Result<Uri, UriError> {
        format!("http://{}:{}/api/v0", host, port).parse()
    }

    /// Generic method for making a request to the client, and getting
    /// a deserializable response.
    ///
    fn request<Req, Res>(&self, req: &Req) -> AsyncResponse<Res>
    where
        Req: ApiRequest,
        for<'de> Res: 'static + Deserialize<'de>,
    {
        let uri = format!("{}{}?", self.base, Req::path()).parse().unwrap();

        Box::new(
            self.client
                .get(uri)
                .and_then(|res| res.body().concat2())
                .from_err()
                .and_then(move |body| {
                    serde_json::from_slice(&body).map_err(From::from)
                }),
        )
    }
}

impl IpfsClient {
    /// List the contents of an Ipfs multihash.
    ///
    #[inline]
    pub fn ls(&self, path: Option<&str>) -> AsyncResponse<response::LsResponse> {
        self.request(&request::LsRequest(path))
    }

    /// Returns information about the Ipfs server version.
    ///
    #[inline]
    pub fn version(&self) -> AsyncResponse<response::VersionResponse> {
        self.request(&request::Version)
    }
}
