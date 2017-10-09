use reqwest;
use serde_json;
use std::fmt::{self, Display, Formatter};
use std::string::FromUtf8Error;


#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    Parse(serde_json::Error),
    ParseUtf8(FromUtf8Error),
    Url(reqwest::UrlError),
    Api(ApiError),
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
