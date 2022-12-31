// Copyright 2021 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::{
    header::{TRAILER, X_STREAM_ERROR_KEY},
    read::{JsonLineDecoder, StreamReader},
    ApiError, ApiRequest,
};
use async_trait::async_trait;
use bytes::Bytes;
use common_multipart_rfc7578::client::multipart;
use futures::{future, FutureExt, Stream, TryStreamExt};
use http::{
    header::{HeaderName, HeaderValue},
    StatusCode,
};
use serde::Deserialize;
use std::fmt::Display;
use tokio_util::codec::{Decoder, FramedRead};

cfg_if::cfg_if! {
    if #[cfg(feature = "with-send-sync")] {
        pub type BoxStream<T, E> = Box<dyn Stream<Item = Result<T, E>> + Send + Unpin>;
    } else {
        pub type BoxStream<T, E> = Box<dyn Stream<Item = Result<T, E>> + Unpin>;
    }
}

#[cfg_attr(feature = "with-send-sync", async_trait)]
#[cfg_attr(not(feature = "with-send-sync"), async_trait(?Send))]
pub trait Backend {
    cfg_if::cfg_if! {
        if #[cfg(feature = "with-send-sync")] {
            type HttpRequest: Send;
        } else {
            type HttpRequest;
        }
    }

    type HttpResponse;

    cfg_if::cfg_if! {
        if #[cfg(feature = "with-send-sync")] {
            type Error: Display + From<ApiError> + From<crate::Error> + Send + 'static;
        } else {
            type Error: Display + From<ApiError> + From<crate::Error> + 'static;
        }
    }

    /// Builds the url for an api call.
    ///
    fn build_base_request<Req>(
        &self,
        req: Req,
        form: Option<multipart::Form<'static>>,
    ) -> Result<Self::HttpRequest, Self::Error>
    where
        Req: ApiRequest;

    /// Get the value of a header from an HTTP response.
    ///
    fn get_header(res: &Self::HttpResponse, key: HeaderName) -> Option<&HeaderValue>;

    /// Generates a request, and returns the unprocessed response future.
    ///
    async fn request_raw<Req>(
        &self,
        req: Req,
        form: Option<multipart::Form<'static>>,
    ) -> Result<(StatusCode, Bytes), Self::Error>
    where
        Req: ApiRequest;

    fn response_to_byte_stream(res: Self::HttpResponse) -> BoxStream<Bytes, Self::Error>;

    /// Generic method for making a request that expects back a streaming
    /// response.
    ///
    fn request_stream<Res, F>(
        &self,
        req: Self::HttpRequest,
        process: F,
    ) -> BoxStream<Res, Self::Error>
    where
        F: 'static + Send + Fn(Self::HttpResponse) -> BoxStream<Res, Self::Error>;

    /// Builds an Api error from a response body.
    ///
    #[inline]
    fn process_error_from_body(body: Bytes) -> Self::Error {
        match serde_json::from_slice::<ApiError>(&body) {
            Ok(e) => e.into(),
            Err(_) => {
                let err = match String::from_utf8(body.to_vec()) {
                    Ok(s) => crate::Error::UnrecognizedApiError(s),
                    Err(e) => crate::Error::from(e),
                };

                err.into()
            }
        }
    }

    /// Processes a response that expects a json encoded body, returning an
    /// error or a deserialized json response.
    ///
    fn process_json_response<Res>(status: StatusCode, body: Bytes) -> Result<Res, Self::Error>
    where
        for<'de> Res: 'static + Deserialize<'de> + Send,
    {
        match status {
            StatusCode::OK => serde_json::from_slice(&body)
                .map_err(crate::Error::from)
                .map_err(Self::Error::from),
            _ => Err(Self::process_error_from_body(body)),
        }
    }

    /// Processes a response that returns a stream of json deserializable
    /// results.
    ///
    fn process_stream_response<D, Res>(
        res: Self::HttpResponse,
        decoder: D,
    ) -> FramedRead<StreamReader<BoxStream<Bytes, Self::Error>>, D>
    where
        D: Decoder<Item = Res, Error = crate::Error>,
    {
        FramedRead::new(
            StreamReader::new(Self::response_to_byte_stream(res)),
            decoder,
        )
    }

    /// Generic method for making a request to the Ipfs server, and getting
    /// a deserializable response.
    ///
    async fn request<Req, Res>(
        &self,
        req: Req,
        form: Option<multipart::Form<'static>>,
    ) -> Result<Res, Self::Error>
    where
        Req: ApiRequest,
        for<'de> Res: 'static + Deserialize<'de> + Send,
    {
        let (status, chunk) = self.request_raw(req, form).await?;

        Self::process_json_response(status, chunk)
    }

    /// Generic method for making a request to the Ipfs server, and getting
    /// back a response with no body.
    ///
    async fn request_empty<Req>(
        &self,
        req: Req,
        form: Option<multipart::Form<'static>>,
    ) -> Result<(), Self::Error>
    where
        Req: ApiRequest,
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
    ) -> Result<String, Self::Error>
    where
        Req: ApiRequest,
    {
        let (status, chunk) = self.request_raw(req, form).await?;

        match status {
            StatusCode::OK => String::from_utf8(chunk.to_vec())
                .map_err(crate::Error::from)
                .map_err(Self::Error::from),
            _ => Err(Self::process_error_from_body(chunk)),
        }
    }

    /// Generic method for making a request to the Ipfs server, and getting
    /// back a raw stream of bytes.
    ///
    fn request_stream_bytes(&self, req: Self::HttpRequest) -> BoxStream<Bytes, Self::Error> {
        self.request_stream(req, |res| Self::response_to_byte_stream(res))
    }

    /// Generic method to return a streaming response of deserialized json
    /// objects delineated by new line separators.
    ///
    fn request_stream_json<Res>(&self, req: Self::HttpRequest) -> BoxStream<Res, Self::Error>
    where
        for<'de> Res: 'static + Deserialize<'de> + Send,
    {
        self.request_stream(req, |res| {
            let parse_stream_error = if let Some(trailer) = Self::get_header(&res, TRAILER) {
                // Response has the Trailer header set. The StreamError trailer
                // is used to indicate that there was an error while streaming
                // data with Ipfs.
                //
                if trailer == X_STREAM_ERROR_KEY {
                    true
                } else {
                    let err = crate::Error::UnrecognizedTrailerHeader(
                        String::from_utf8_lossy(trailer.as_ref()).into(),
                    );

                    // There was an unrecognized trailer value. If that is the case,
                    // create a stream that immediately errors.
                    //
                    return Box::new(future::err(err).into_stream().err_into());
                }
            } else {
                false
            };

            Box::new(
                Self::process_stream_response(res, JsonLineDecoder::new(parse_stream_error))
                    .err_into(),
            )
        })
    }

    /// Set basic authentication credentials to use on every request from this client.
    fn with_credentials<U, P>(self, username: U, password: P) -> Self
    where
        U: Into<String>,
        P: Into<String>;
}
