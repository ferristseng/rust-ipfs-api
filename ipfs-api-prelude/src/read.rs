// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::header::X_STREAM_ERROR;
use bytes::{Bytes, BytesMut};
use futures::{
    ready,
    task::{Context, Poll},
    Stream,
};
use serde::Deserialize;
use std::{cmp, fmt::Display, io, marker::PhantomData, pin::Pin};
use tokio::io::{AsyncRead, ReadBuf};
use tokio_util::codec::Decoder;
use tracing::{event, instrument, Level};

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

    type Error = crate::Error;

    /// Tries to find a new line character. If it does, it will split the buffer,
    /// and parse the first slice.
    ///
    #[instrument(skip(self, src), fields(stream_trailer = self.parse_stream_error))]
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let nl_index = src.iter().position(|b| *b == b'\n');

        if let Some(pos) = nl_index {
            event!(Level::INFO, "Found new line delimeter in buffer");

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
                            Some(colon) if &slice[..colon] == X_STREAM_ERROR.as_bytes() => {
                                let e = crate::Error::StreamError(
                                    String::from_utf8_lossy(&slice[colon + 2..]).into(),
                                );

                                Err(e)
                            }
                            _ => Err(e.into()),
                        }
                    } else {
                        Err(e.into())
                    }
                }
            }
        } else {
            event!(Level::INFO, "Waiting for more data to decode JSON");

            Ok(None)
        }
    }
}

/// A decoder that reads a line at a time.
///
pub struct LineDecoder;

impl Decoder for LineDecoder {
    type Item = String;

    type Error = crate::Error;

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

/// Copies bytes from a Bytes chunk into a destination buffer, and returns
/// the number of bytes that were read.
///
fn copy_from_chunk_to(dest: &mut ReadBuf<'_>, chunk: &mut Bytes, chunk_start: usize) -> usize {
    let len = cmp::min(dest.capacity(), chunk.len() - chunk_start);
    let chunk_end = chunk_start + len;

    dest.put_slice(&chunk[chunk_start..chunk_end]);

    len
}

/// The state of a stream returning Chunks.
///
enum ReadState {
    /// A chunk is ready to be read from.
    ///
    Ready(Bytes, usize),

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

impl<S, E> StreamReader<S>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Display,
{
    #[inline]
    pub fn new(stream: S) -> StreamReader<S> {
        StreamReader {
            stream,
            state: ReadState::NotReady,
        }
    }
}

impl<S, E> AsyncRead for StreamReader<S>
where
    S: Stream<Item = Result<Bytes, E>> + Unpin,
    E: Display,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        match self.state {
            // Stream yielded a Chunk to read.
            //
            ReadState::Ready(ref mut chunk, ref mut pos) => {
                let bytes_read = copy_from_chunk_to(buf, chunk, *pos);

                if *pos + bytes_read >= chunk.len() {
                    self.state = ReadState::NotReady;
                } else {
                    *pos += bytes_read;
                }

                Poll::Ready(Ok(()))
            }
            // Stream is not ready, and a Chunk needs to be read.
            //
            ReadState::NotReady => {
                match ready!(Stream::poll_next(Pin::new(&mut self.stream), cx)) {
                    // Polling stream yielded a Chunk that can be read from.
                    //
                    Some(Ok(mut chunk)) => {
                        let bytes_read = copy_from_chunk_to(buf, &mut chunk, 0);

                        if bytes_read >= chunk.len() {
                            self.state = ReadState::NotReady;
                        } else {
                            self.state = ReadState::Ready(chunk, bytes_read);
                        }

                        Poll::Ready(Ok(()))
                    }
                    Some(Err(e)) => {
                        Poll::Ready(Err(io::Error::new(io::ErrorKind::Other, e.to_string())))
                    }
                    // Polling stream yielded EOF.
                    //
                    None => Poll::Ready(Ok(())),
                }
            }
        }
    }
}
