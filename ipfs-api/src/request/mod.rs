pub use self::add::*;
pub use self::bootstrap::*;
pub use self::commands::*;
pub use self::config::*;
pub use self::ls::*;
pub use self::stats::*;
pub use self::swarm::*;
pub use self::version::*;


mod add;
mod bootstrap;
mod commands;
mod config;
mod ls;
mod stats;
mod swarm;
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
