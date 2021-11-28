use crate::{request::ApiRequest, Backend};
use async_trait::async_trait;
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
struct OptCombiner<'a, Req> {
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

    fn combine<'a, Req>(&'a self, req: Req) -> OptCombiner<'a, Req>
    where
        Req: ApiRequest + Send,
    {
        OptCombiner {
            global: &self.options,
            request: req,
        }
    }
}

impl<'a, Req: ApiRequest> ApiRequest for OptCombiner<'a, Req> {
    const PATH: &'static str = <Req as ApiRequest>::PATH;

    const METHOD: http::Method = http::Method::POST;
}

#[async_trait]
impl<Back: Backend + Sync> Backend for BackendWithGlobalOptions<Back> {
    type HttpRequest = Back::HttpRequest;

    type HttpResponse = Back::HttpResponse;

    type Error = Back::Error;

    fn build_base_request<Req>(
        &self,
        req: Req,
        form: Option<common_multipart_rfc7578::client::multipart::Form<'static>>,
    ) -> Result<Self::HttpRequest, Self::Error>
    where
        Req: ApiRequest + Send,
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
        form: Option<common_multipart_rfc7578::client::multipart::Form<'static>>,
    ) -> Result<(http::StatusCode, bytes::Bytes), Self::Error>
    where
        Req: ApiRequest + Serialize + Send,
    {
        let request = self.backend.request_raw(self.combine(req), form);

        request.await
    }

    fn response_to_byte_stream(
        res: Self::HttpResponse,
    ) -> Box<dyn futures::Stream<Item = Result<bytes::Bytes, Self::Error>> + Send + Unpin> {
        Back::response_to_byte_stream(res)
    }

    fn request_stream<Res, F, OutStream>(
        &self,
        req: Self::HttpRequest,
        process: F,
    ) -> Box<dyn futures::Stream<Item = Result<Res, Self::Error>> + Send + Unpin>
    where
        OutStream: futures::Stream<Item = Result<Res, Self::Error>> + Send + Unpin,
        F: 'static + Send + Fn(Self::HttpResponse) -> OutStream,
    {
        self.backend.request_stream(req, process)
    }
}
