// Copyright 2022 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::error::Error;
use async_trait::async_trait;
use bytes::Bytes;
use futures::{FutureExt, StreamExt, TryFutureExt, TryStreamExt};
use http::{
    header::{HeaderName, HeaderValue},
    uri::Scheme,
    StatusCode, Uri,
};
use hyper::{
    body,
    client::{self, connect::Connect, Builder, HttpConnector},
};
use ipfs_api_prelude::{ApiRequest, Backend, BoxStream, TryFromUri};
use multipart::client::multipart;

macro_rules! impl_default {
    ($http_connector:path) => {
        impl_default!($http_connector, <$http_connector>::new());
    };
    ($http_connector:path, $constructor:expr) => {
        #[derive(Clone)]
        pub struct HyperBackend<C = $http_connector>
        where
            C: Connect + Clone + Send + Sync + 'static,
        {
            base: Uri,
            client: client::Client<C, hyper::Body>,

            /// Username and password
            credentials: Option<(String, String)>,
        }

        impl Default for HyperBackend<$http_connector> {
            /// Creates an `IpfsClient` connected to the endpoint specified in ~/.ipfs/api.
            /// If not found, tries to connect to `localhost:5001`.
            ///
            fn default() -> Self {
                Self::from_ipfs_config().unwrap_or_else(|| {
                    Self::from_host_and_port(Scheme::HTTP, "localhost", 5001).unwrap()
                })
            }
        }

        impl TryFromUri for HyperBackend<$http_connector> {
            fn build_with_base_uri(base: Uri) -> Self {
                let client = Builder::default()
                    .pool_max_idle_per_host(0)
                    .build($constructor);

                HyperBackend {
                    base,
                    client,
                    credentials: None,
                }
            }
        }
    };
}

// Because the Hyper TLS connector supports both HTTP and HTTPS,
// if TLS is enabled, always use the TLS connector as default.
//
// Otherwise, compile errors will result due to ambiguity:
//
//   * "cannot infer type for struct `IpfsClient<_>`"
//
#[cfg(not(feature = "with-hyper-tls"))]
#[cfg(not(feature = "with-hyper-rustls"))]
impl_default!(HttpConnector);

#[cfg(feature = "with-hyper-tls")]
impl_default!(hyper_tls::HttpsConnector<HttpConnector>);

#[cfg(feature = "with-hyper-rustls")]
impl_default!(
    hyper_rustls::HttpsConnector<HttpConnector>,
    hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_or_http()
        .enable_http1()
        .build()
);

impl<C: Connect + Clone + Send + Sync + 'static> HyperBackend<C> {
    pub fn with_credentials<U, P>(self, username: U, password: P) -> Self
    where
        U: Into<String>,
        P: Into<String>,
    {
        Self {
            base: self.base,
            client: self.client,
            credentials: Some((username.into(), password.into())),
        }
    }

    fn basic_authorization(&self) -> Option<String> {
        self.credentials.as_ref().map(|(username, password)| {
            let credentials = format!("{}:{}", username, password);
            let encoded = base64::encode(credentials);

            format!("Basic {}", encoded)
        })
    }
}

#[cfg_attr(feature = "with-send-sync", async_trait)]
#[cfg_attr(not(feature = "with-send-sync"), async_trait(?Send))]
impl<C> Backend for HyperBackend<C>
where
    C: Connect + Clone + Send + Sync + 'static,
{
    type HttpRequest = http::Request<hyper::Body>;

    type HttpResponse = http::Response<hyper::Body>;

    type Error = Error;

    fn with_credentials<U, P>(self, username: U, password: P) -> Self
    where
        U: Into<String>,
        P: Into<String>,
    {
        (self as HyperBackend<C>).with_credentials(username, password)
    }

    fn build_base_request<Req>(
        &self,
        req: Req,
        form: Option<multipart::Form<'static>>,
    ) -> Result<Self::HttpRequest, Error>
    where
        Req: ApiRequest,
    {
        let url = req.absolute_url(&self.base)?;

        let builder = http::Request::builder();
        let builder = builder.method(Req::METHOD).uri(url);

        let builder = if let Some(authorization) = self.basic_authorization() {
            builder.header(hyper::header::AUTHORIZATION, authorization)
        } else {
            builder
        };

        let req = if let Some(form) = form {
            form.set_body_convert::<hyper::Body, multipart::Body>(builder)
        } else {
            builder.body(hyper::Body::empty())
        }?;

        Ok(req)
    }

    fn get_header(res: &Self::HttpResponse, key: HeaderName) -> Option<&HeaderValue> {
        res.headers().get(key)
    }

    async fn request_raw<Req>(
        &self,
        req: Req,
        form: Option<multipart::Form<'static>>,
    ) -> Result<(StatusCode, Bytes), Self::Error>
    where
        Req: ApiRequest,
    {
        let req = self.build_base_request(req, form)?;
        let res = self.client.request(req).await?;
        let status = res.status();
        let body = body::to_bytes(res.into_body()).await?;

        Ok((status, body))
    }

    fn response_to_byte_stream(res: Self::HttpResponse) -> BoxStream<Bytes, Self::Error> {
        Box::new(res.into_body().err_into())
    }

    fn request_stream<Res, F>(
        &self,
        req: Self::HttpRequest,
        process: F,
    ) -> BoxStream<Res, Self::Error>
    where
        F: 'static + Send + Fn(Self::HttpResponse) -> BoxStream<Res, Self::Error>,
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
