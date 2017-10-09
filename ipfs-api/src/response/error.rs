use reqwest;
use serde_json;
use std::fmt::{self, Display, Formatter};


#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    Parse(serde_json::Error),
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
            Error::Parse(_) => "an error occursed while parsing the api response",
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
