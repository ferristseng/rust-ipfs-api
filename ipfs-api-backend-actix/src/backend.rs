// Copyright 2021 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::error::Error;
use async_trait::async_trait;
use awc::Client;
use bytes::Bytes;
use futures::{FutureExt, StreamExt, TryFutureExt, TryStreamExt};
use http::{
    header::{HeaderName, HeaderValue},
    uri::Scheme,
    StatusCode, Uri,
};
use ipfs_api_prelude::{ApiRequest, Backend, BoxStream, TryFromUri};
use multipart::client::multipart;
use std::{borrow::Borrow, time::Duration};

const ACTIX_REQUEST_TIMEOUT: Duration = Duration::from_secs(90);

pub struct ActixBackend {
    base: Uri,
    client: Client,

    /// Username and password
    credentials: Option<(String, String)>,
}

impl Default for ActixBackend {
    /// Creates an `IpfsClient` connected to the endpoint specified in ~/.ipfs/api.
    /// If not found, tries to connect to `localhost:5001`.
    ///
    fn default() -> Self {
        Self::from_ipfs_config()
            .unwrap_or_else(|| Self::from_host_and_port(Scheme::HTTP, "localhost", 5001).unwrap())
    }
}

impl TryFromUri for ActixBackend {
    fn build_with_base_uri(base: Uri) -> Self {
        let client = Client::default();

        ActixBackend {
            base,
            client,
            credentials: None,
        }
    }
}

impl ActixBackend {
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
}

// Pending until https://github.com/actix/actix-web/issues/3384
fn to_http_0_2(method: http::Method) -> http_02::Method {
    match method {
        http::Method::POST => http_02::Method::POST,
        _ => todo!("Not used by codebase"),
    }
}

#[async_trait(?Send)]
impl Backend for ActixBackend {
    type HttpRequest = awc::SendClientRequest;

    type HttpResponse = awc::ClientResponse<actix_http::encoding::Decoder<actix_http::Payload>>;

    type Error = Error;

    fn with_credentials<U, P>(self, username: U, password: P) -> Self
    where
        U: Into<String>,
        P: Into<String>,
    {
        (self as ActixBackend).with_credentials(username, password)
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
        let req = self
            .client
            .request(to_http_0_2(Req::METHOD), url.to_string());
        let req = if let Some((username, password)) = &self.credentials {
            req.basic_auth(username, password)
        } else {
            req
        };
        let req = if let Some(form) = form {
            req.content_type(form.content_type())
                .send_body(multipart::Body::from(form))
        } else {
            req.timeout(ACTIX_REQUEST_TIMEOUT).send()
        };

        Ok(req)
    }

    fn get_header(res: &Self::HttpResponse, key: HeaderName) -> Option<impl Borrow<HeaderValue>> {
        // mapping is needed until https://github.com/actix/actix-web/issues/3384 is done
        res.headers()
            .get(key.as_str())
            .and_then(|v| HeaderValue::from_bytes(v.clone().as_bytes()).ok())
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
        let mut res = req.await?;
        let status = res.status();
        let body = res.body().await?;

        // FIXME: Actix compat with bytes 1.0
        Ok((
            // mapping is needed until https://github.com/actix/actix-web/issues/3384 is done
            StatusCode::from_u16(status.as_u16())
                .expect("failed mapping http 0.2 to http 1.0 status code"),
            body,
        ))
    }

    fn response_to_byte_stream(res: Self::HttpResponse) -> BoxStream<Bytes, Self::Error> {
        let stream = res.err_into();

        Box::new(stream)
    }

    fn request_stream<Res, F>(
        &self,
        req: Self::HttpRequest,
        process: F,
    ) -> BoxStream<Res, Self::Error>
    where
        F: 'static + Send + Fn(Self::HttpResponse) -> BoxStream<Res, Self::Error>,
    {
        let stream = req
            .err_into()
            .map_ok(move |mut res| {
                match res.status() {
                    http_02::StatusCode::OK => process(res).right_stream(),
                    // If the server responded with an error status code, the body
                    // still needs to be read so an error can be built. This block will
                    // read the entire body stream, then immediately return an error.
                    //
                    _ => res
                        .body()
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
