use reqwest;
use serde_json;
use serde_urlencoded;
use std::fmt::{self, Display, Formatter};
use std::string::FromUtf8Error;


#[derive(Debug)]
pub enum Error {
    /// Error when making HTTP request to server.
    ///
    Http(reqwest::Error),

    /// Error when parsing a JSON response.
    ///
    Parse(serde_json::Error),

    /// Error when parsing a raw UTF8 string response.
    ///
    ParseUtf8(FromUtf8Error),

    /// Error when parsing the request URL.
    ///
    Url(reqwest::UrlError),

    /// Error when encoding the URL query string parameters.
    ///
    EncodeUrl(serde_urlencoded::ser::Error),

    /// Error returned by the Ipfs API.
    ///
    Api(ApiError),

    /// Unknown error.
    ///
    Uncategorized(String),
}

impl From<reqwest::UrlError> for Error {
    fn from(error: reqwest::UrlError) -> Self {
        Error::Url(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Http(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Parse(error)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Error::ParseUtf8(error)
    }
}

impl From<serde_urlencoded::ser::Error> for Error {
    fn from(error: serde_urlencoded::ser::Error) -> Self {
        Error::EncodeUrl(error)
    }
}

impl From<ApiError> for Error {
    fn from(error: ApiError) -> Self {
        Error::Api(error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Http(_) => "an http error occured",
            Error::Parse(_) => "an error occured while parsing the api response",
            Error::ParseUtf8(_) => "an error occured while parsing a string response from the api",
            Error::Url(_) => "an error occured while parsing the request url",
            Error::EncodeUrl(_) => "an error occured while encoding the request parameters",
            Error::Api(_) => "an api error occured",
            Error::Uncategorized(_) => "an unknown error occured",
        }
    }
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApiError {
    pub message: String,
    pub code: u8,
}
