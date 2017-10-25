use reqwest;
use serde_json;
use serde_urlencoded;
use std::string::FromUtf8Error;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ApiError {
    pub message: String,
    pub code: u8,
}


error_chain! {
    foreign_links {
        Http(reqwest::Error);
        Parse(serde_json::Error);
        ParseUtf8(FromUtf8Error);
        Url(reqwest::UrlError);
        Io(::std::io::Error);
        EncodeUrl(serde_urlencoded::ser::Error);
    }

    errors {
        /// An error returned by the Ipfs api.
        ///
        Api(err: ApiError) {
            description("api returned an error"),
            display("api returned '{}'", err.message)
        }

        Uncategorized(err: String) {
            description("api returned an unknown error"),
            display("api returned '{}'", err)
        }
    }
}
