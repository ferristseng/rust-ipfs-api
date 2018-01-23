// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use bytes::BytesMut;
use futures::{Async, Stream};
use header::XStreamError;
use hyper::Chunk;
use hyper::header::{Header, Raw};
use response::{Error, ErrorKind};
use serde::Deserialize;
use serde_json;
use std::cmp;
use std::io::{self, Read};
use std::marker::PhantomData;
use tokio_io::AsyncRead;
use tokio_io::codec::Decoder;

/// A decoder for a response where each line is a full json object.
///
pub struct JsonLineDecoder<T> {
    /// Set to true if the stream can contain a X-Stream-Error header,
    /// which indicates an error while streaming.
    ///
    parse_stream_error: bool,

    ty: PhantomData<T>,
}

impl<T> JsonLineDecoder<T> {
    #[inline]
    pub fn new(parse_stream_error: bool) -> JsonLineDecoder<T> {
        JsonLineDecoder {
            parse_stream_error,
            ty: PhantomData,
        }
    }
}

impl<T> Decoder for JsonLineDecoder<T>
where
    for<'de> T: Deserialize<'de>,
{
    type Item = T;

    type Error = Error;

    /// Tries to find a new line character. If it does, it will split the buffer,
    /// and parse the first slice.
    ///
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let nl_index = src.iter().position(|b| *b == b'\n');

        if let Some(pos) = nl_index {
            let slice = src.split_to(pos + 1);
            let slice = &slice[..slice.len() - 1];

            match serde_json::from_slice(slice) {
                Ok(json) => Ok(json),
                // If a JSON object couldn't be parsed from the response, it is possible
                // that a stream error trailing header was returned. If the JSON decoder
                // was configured to parse these kinds of error, it should try. If a header
                // couldn't be parsed, it will return the original error.
                //
                Err(e) => {
                    if self.parse_stream_error {
                        match slice.iter().position(|&x| x == b':') {
                            Some(colon)
                                if &slice[..colon] == XStreamError::header_name().as_bytes() =>
                            {
                                let raw = Raw::from(&slice[colon + 2..]);

                                match XStreamError::parse_header(&raw) {
                                    Ok(stream_error) => {
                                        Err(ErrorKind::StreamError(stream_error.error).into())
                                    }
                                    Err(_) => Err(e.into()),
                                }
                            }
                            _ => Err(e.into()),
                        }
                    } else {
                        Err(e.into())
                    }
                }
            }
        } else {
            Ok(None)
        }
    }
}

/// A decoder that reads a line at a time.
///
pub struct LineDecoder;

impl Decoder for LineDecoder {
    type Item = String;

    type Error = Error;

    /// Attempts to find a new line character, and returns the entire line if
    /// it finds one.
    ///
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let nl_index = src.iter().position(|b| *b == b'\n');

        if let Some(pos) = nl_index {
            let slice = src.split_to(pos + 1);

            Ok(Some(
                String::from_utf8_lossy(&slice[..slice.len() - 1]).into_owned(),
            ))
        } else {
            Ok(None)
        }
    }
}

/// The state of a stream returning Chunks.
///
enum ReadState {
    /// A chunk is ready to be read from.
    ///
    Ready(Chunk, usize),

    /// The next chunk isn't ready yet.
    ///
    NotReady,
}

/// Reads from a stream of chunks asynchronously.
///
pub struct StreamReader<S> {
    stream: S,
    state: ReadState,
}

impl<S> StreamReader<S>
where
    S: Stream<Item = Chunk, Error = Error>,
{
    #[inline]
    pub fn new(stream: S) -> StreamReader<S> {
        StreamReader {
            stream: stream,
            state: ReadState::NotReady,
        }
    }
}

impl<S> Read for StreamReader<S>
where
    S: Stream<Item = Chunk, Error = Error>,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            let ret;

            match self.state {
                // Stream yielded a Chunk to read.
                //
                ReadState::Ready(ref mut chunk, ref mut pos) => {
                    let chunk_start = *pos;
                    let len = cmp::min(buf.len(), chunk.len() - chunk_start);
                    let chunk_end = chunk_start + len;

                    buf[..len].copy_from_slice(&chunk[chunk_start..chunk_end]);
                    *pos += len;

                    if *pos == chunk.len() {
                        ret = len;
                    } else {
                        return Ok(len);
                    }
                }
                // Stream is not ready, and a Chunk needs to be read.
                //
                ReadState::NotReady => {
                    match self.stream.poll() {
                        // Polling stream yielded a Chunk that can be read from.
                        //
                        Ok(Async::Ready(Some(chunk))) => {
                            self.state = ReadState::Ready(chunk, 0);

                            continue;
                        }
                        // Polling stream yielded EOF.
                        //
                        Ok(Async::Ready(None)) => return Ok(0),
                        // Stream could not be read from.
                        //
                        Ok(Async::NotReady) => return Err(io::ErrorKind::WouldBlock.into()),
                        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.description())),
                    }
                }
            }

            self.state = ReadState::NotReady;

            return Ok(ret);
        }
    }
}

impl<S> AsyncRead for StreamReader<S>
where
    S: Stream<Item = Chunk, Error = Error>,
{
}
