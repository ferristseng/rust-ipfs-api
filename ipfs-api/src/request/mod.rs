pub use self::commands::*;
pub use self::ls::*;
pub use self::version::*;


mod commands;
mod ls;
mod version;


/// A request that can be made against the Ipfs API.
///
pub trait ApiRequest {
    /// Returns the API path that this request can be called on.
    ///
    /// All paths should begin with '/'.
    ///
    fn path() -> &'static str;
}
