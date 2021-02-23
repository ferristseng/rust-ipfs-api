use crate::error::Error;
use async_trait::async_trait;
use bytes::Bytes;
use futures::{FutureExt, Stream, StreamExt, TryFutureExt, TryStreamExt};
use http::{
    header::{HeaderName, HeaderValue},
    uri::Scheme,
    StatusCode, Uri,
};
use hyper::{
    body,
    client::{self, Builder, HttpConnector},
};
use hyper_tls::HttpsConnector;
use ipfs_api_prelude::{ApiRequest, Backend, TryFromUri};
use multipart::client::multipart;
use serde::Serialize;

pub struct HyperBackend {
    base: Uri,
    client: client::Client<HttpsConnector<HttpConnector>, hyper::Body>,
}

impl Default for HyperBackend {
    fn default() -> Self {
        Self::from_ipfs_config()
            .unwrap_or_else(|| Self::from_host_and_port(Scheme::HTTP, "localhost", 5001).unwrap())
    }
}

impl TryFromUri for HyperBackend {
    fn build_with_base_uri(base: Uri) -> Self {
        let client = Builder::default()
            .pool_max_idle_per_host(0)
            .build(HttpsConnector::new());

        HyperBackend { base, client }
    }
}

#[async_trait(?Send)]
impl Backend for HyperBackend {
    type HttpRequest = http::Request<hyper::Body>;

    type HttpResponse = http::Response<hyper::Body>;

    type Error = Error;

    fn build_base_request<Req>(
        &self,
        req: &Req,
        form: Option<multipart::Form<'static>>,
    ) -> Result<Self::HttpRequest, Error>
    where
        Req: ApiRequest,
    {
        let url = req.absolute_url(&self.base)?;

        let builder = http::Request::builder();
        let builder = builder.method(Req::METHOD.clone()).uri(url);

        let req = if let Some(form) = form {
            form.set_body_convert::<hyper::Body, multipart::Body>(builder)
        } else {
            builder.body(hyper::Body::empty())
        }?;

        Ok(req)
    }

    fn get_header<'a>(res: &'a Self::HttpResponse, key: HeaderName) -> Option<&'a HeaderValue> {
        res.headers().get(key)
    }

    async fn request_raw<Req>(
        &self,
        req: Req,
        form: Option<multipart::Form<'static>>,
    ) -> Result<(StatusCode, Bytes), Self::Error>
    where
        Req: ApiRequest + Serialize,
    {
        let req = self.build_base_request(&req, form)?;
        let res = self.client.request(req).await?;
        let status = res.status();
        let body = body::to_bytes(res.into_body()).await?;

        Ok((status, body))
    }

    fn response_to_byte_stream(
        res: Self::HttpResponse,
    ) -> Box<dyn Stream<Item = Result<Bytes, Self::Error>> + Unpin> {
        Box::new(res.into_body().err_into())
    }

    fn request_stream<Res, F, OutStream>(
        &self,
        req: Self::HttpRequest,
        process: F,
    ) -> Box<dyn Stream<Item = Result<Res, Self::Error>> + Unpin>
    where
        OutStream: Stream<Item = Result<Res, Self::Error>> + Unpin,
        F: 'static + Fn(Self::HttpResponse) -> OutStream,
    {
        let stream = self
            .client
            .request(req)
            .err_into()
            .map_ok(move |res| {
                match res.status() {
                    StatusCode::OK => process(res).right_stream(),
                    // If the server responded with an error status code, the body
                    // still needs to be read so an error can be built. This block will
                    // read the entire body stream, then immediately return an error.
                    //
                    _ => body::to_bytes(res.into_body())
                        .boxed()
                        .map(|maybe_body| match maybe_body {
                            Ok(body) => Err(Self::process_error_from_body(body)),
                            Err(e) => Err(e.into()),
                        })
                        .into_stream()
                        .left_stream(),
                }
            })
            .try_flatten_stream();

        Box::new(stream)
    }
}
