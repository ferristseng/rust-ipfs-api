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
use futures::{future, FutureExt, Stream, StreamExt, TryStreamExt};
use http::{
    header::{HeaderName, HeaderValue},
    StatusCode,
};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, string::FromUtf8Error};
use tokio_util::codec::{Decoder, FramedRead};

#[async_trait(?Send)]
pub trait Backend: Default {
    /// HTTP request type.
    ///
    type HttpRequest;

    /// HTTP response type.
    ///
    type HttpResponse;

    /// HTTP multipart form type.
    ///
    type MultipartForm: Default;

    /// Error type for Result.
    ///
    type Error: Display + From<ApiError> + From<crate::Error> + 'static;

    fn build_base_request<Req>(
        &self,
        req: &Req,
        form: Option<Self::MultipartForm>,
    ) -> Result<Self::HttpRequest, Self::Error>
    where
        Req: ApiRequest;

    fn get_header<'a>(res: &'a Self::HttpResponse, key: HeaderName) -> Option<&'a HeaderValue>;

    async fn request_raw<Req>(
        &self,
        req: Req,
        form: Option<Self::MultipartForm>,
    ) -> Result<(StatusCode, Bytes), Self::Error>
    where
        Req: ApiRequest + Serialize;

    fn response_to_byte_stream(
        res: Self::HttpResponse,
    ) -> Box<dyn Stream<Item = Result<Bytes, Self::Error>> + Unpin>;

    fn request_stream<Res, F, OutStream>(
        &self,
        req: Self::HttpRequest,
        process: F,
    ) -> Box<dyn Stream<Item = Result<Res, Self::Error>> + Unpin>
    where
        OutStream: Stream<Item = Result<Res, Self::Error>> + Unpin,
        F: 'static + Fn(Self::HttpResponse) -> OutStream;

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
        for<'de> Res: 'static + Deserialize<'de>,
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
    ) -> FramedRead<StreamReader<Box<dyn Stream<Item = Result<Bytes, Self::Error>> + Unpin>>, D>
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
        form: Option<Self::MultipartForm>,
    ) -> Result<Res, Self::Error>
    where
        Req: ApiRequest + Serialize,
        for<'de> Res: 'static + Deserialize<'de>,
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
        form: Option<Self::MultipartForm>,
    ) -> Result<(), Self::Error>
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
        form: Option<Self::MultipartForm>,
    ) -> Result<String, Self::Error>
    where
        Req: ApiRequest + Serialize,
    {
        let (status, chunk) = self.request_raw(req, form).await?;

        match status {
            StatusCode::OK => String::from_utf8(chunk.to_vec())
                .map_err(crate::Error::from)
                .map_err(Self::Error::from),
            _ => Err(Self::process_error_from_body(chunk)),
        }
    }

    fn request_stream_bytes(
        &self,
        req: Self::HttpRequest,
    ) -> Box<dyn Stream<Item = Result<Bytes, Self::Error>> + Unpin> {
        self.request_stream(req, |res| Self::response_to_byte_stream(res))
    }

    /// Generic method to return a streaming response of deserialized json
    /// objects delineated by new line separators.
    ///
    fn request_stream_json<Res>(
        &self,
        req: Self::HttpRequest,
    ) -> Box<dyn Stream<Item = Result<Res, Self::Error>> + Unpin>
    where
        for<'de> Res: 'static + Deserialize<'de>,
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
                    return future::err(err).into_stream().err_into().left_stream();
                }
            } else {
                false
            };

            Self::process_stream_response(res, JsonLineDecoder::new(parse_stream_error))
                .err_into()
                .right_stream()
        })
    }
}
