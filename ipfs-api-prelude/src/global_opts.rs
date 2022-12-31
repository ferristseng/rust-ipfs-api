// Copyright 2021 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::{request::ApiRequest, Backend, BoxStream};
use async_trait::async_trait;
use bytes::Bytes;
use common_multipart_rfc7578::client::multipart;
use serde::{Serialize, Serializer};
use std::time::Duration;

/// Options valid on any IPFS Api request
///
/// Can be set on a client using [BackendWithGlobalOptions]
///
#[cfg_attr(feature = "with-builder", derive(TypedBuilder))]
#[derive(Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct GlobalOptions {
    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    pub offline: Option<bool>,

    #[cfg_attr(feature = "with-builder", builder(default, setter(strip_option)))]
    #[serde(serialize_with = "duration_as_secs_ns")]
    pub timeout: Option<Duration>,
}

fn duration_as_secs_ns<S>(duration: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match duration {
        Some(duration) => serializer.serialize_str(&format!(
            "{}s{}ns",
            duration.as_secs(),
            duration.subsec_nanos(),
        )),
        None => serializer.serialize_none(),
    }
}

/// A wrapper for [Backend] / [IpfsApi](crate::api::IpfsApi) that adds global options
pub struct BackendWithGlobalOptions<Back: Backend> {
    backend: Back,
    options: GlobalOptions,
}

#[derive(Serialize)]
struct OptCombiner<'a, Req>
where
    Req: ApiRequest,
{
    #[serde(flatten)]
    global: &'a GlobalOptions,

    #[serde(flatten)]
    request: Req,
}

impl<Back: Backend> BackendWithGlobalOptions<Back> {
    /// Return the wrapped [Backend]
    pub fn into_inner(self) -> Back {
        self.backend
    }

    /// Construct
    ///
    /// While possible, it is not recommended to wrap a [Backend] twice.
    pub fn new(backend: Back, options: GlobalOptions) -> Self {
        Self { backend, options }
    }

    fn with_credentials<U, P>(self, username: U, password: P) -> Self
    where
        U: Into<String>,
        P: Into<String>,
    {
        Self {
            backend: self.backend.with_credentials(username, password),
            options: self.options,
        }
    }

    fn combine<Req>(&self, req: Req) -> OptCombiner<Req>
    where
        Req: ApiRequest,
    {
        OptCombiner {
            global: &self.options,
            request: req,
        }
    }
}

impl<'a, Req> ApiRequest for OptCombiner<'a, Req>
where
    Req: ApiRequest,
{
    const PATH: &'static str = <Req as ApiRequest>::PATH;

    const METHOD: http::Method = http::Method::POST;
}

#[cfg(feature = "with-send-sync")]
#[async_trait]
impl<Back: Backend + Send + Sync> Backend for BackendWithGlobalOptions<Back> {
    type HttpRequest = Back::HttpRequest;

    type HttpResponse = Back::HttpResponse;

    type Error = Back::Error;

    fn with_credentials<U, P>(self, username: U, password: P) -> Self
    where
        U: Into<String>,
        P: Into<String>,
    {
        (self as BackendWithGlobalOptions<Back>).with_credentials(username, password)
    }

    fn build_base_request<Req>(
        &self,
        req: Req,
        form: Option<multipart::Form<'static>>,
    ) -> Result<Self::HttpRequest, Self::Error>
    where
        Req: ApiRequest,
    {
        self.backend.build_base_request(self.combine(req), form)
    }

    fn get_header(
        res: &Self::HttpResponse,
        key: http::header::HeaderName,
    ) -> Option<&http::HeaderValue> {
        Back::get_header(res, key)
    }

    async fn request_raw<Req>(
        &self,
        req: Req,
        form: Option<multipart::Form<'static>>,
    ) -> Result<(http::StatusCode, bytes::Bytes), Self::Error>
    where
        Req: ApiRequest,
    {
        self.backend.request_raw(self.combine(req), form).await
    }

    fn response_to_byte_stream(res: Self::HttpResponse) -> BoxStream<Bytes, Self::Error> {
        Back::response_to_byte_stream(res)
    }

    fn request_stream<Res, F>(
        &self,
        req: Self::HttpRequest,
        process: F,
    ) -> BoxStream<Res, Self::Error>
    where
        F: 'static + Send + Fn(Self::HttpResponse) -> BoxStream<Res, Self::Error>,
    {
        self.backend.request_stream(req, process)
    }
}

#[cfg(not(feature = "with-send-sync"))]
#[async_trait(?Send)]
impl<Back: Backend> Backend for BackendWithGlobalOptions<Back> {
    type HttpRequest = Back::HttpRequest;

    type HttpResponse = Back::HttpResponse;

    type Error = Back::Error;

    fn with_credentials<U, P>(self, username: U, password: P) -> Self
    where
        U: Into<String>,
        P: Into<String>,
    {
        (self as BackendWithGlobalOptions<Back>).with_credentials(username, password)
    }

    fn build_base_request<Req>(
        &self,
        req: Req,
        form: Option<multipart::Form<'static>>,
    ) -> Result<Self::HttpRequest, Self::Error>
    where
        Req: ApiRequest,
    {
        self.backend.build_base_request(self.combine(req), form)
    }

    fn get_header(
        res: &Self::HttpResponse,
        key: http::header::HeaderName,
    ) -> Option<&http::HeaderValue> {
        Back::get_header(res, key)
    }

    async fn request_raw<Req>(
        &self,
        req: Req,
        form: Option<multipart::Form<'static>>,
    ) -> Result<(http::StatusCode, bytes::Bytes), Self::Error>
    where
        Req: ApiRequest,
    {
        self.backend.request_raw(self.combine(req), form).await
    }

    fn response_to_byte_stream(res: Self::HttpResponse) -> BoxStream<Bytes, Self::Error> {
        Back::response_to_byte_stream(res)
    }

    fn request_stream<Res, F>(
        &self,
        req: Self::HttpRequest,
        process: F,
    ) -> BoxStream<Res, Self::Error>
    where
        F: 'static + Send + Fn(Self::HttpResponse) -> BoxStream<Res, Self::Error>,
    {
        self.backend.request_stream(req, process)
    }
}
