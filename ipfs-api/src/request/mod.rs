pub use self::add::*;
pub use self::commands::*;
pub use self::ls::*;
pub use self::stats::*;
pub use self::version::*;


mod add;
mod commands;
mod ls;
mod stats;
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
