pub use self::add::*;
pub use self::bitswap::*;
pub use self::block::*;
pub use self::bootstrap::*;
pub use self::cat::*;
pub use self::commands::*;
pub use self::config::*;
pub use self::dag::*;
pub use self::ls::*;
pub use self::object::*;
pub use self::pin::*;
pub use self::ping::*;
pub use self::pubsub::*;
pub use self::refs::*;
pub use self::stats::*;
pub use self::swarm::*;
pub use self::version::*;


/// Create a test to verify that serializing a `ApiRequest` returns the expected
/// url encoded string.
///
#[cfg(test)]
macro_rules! serialize_url_test {
    ($f:ident, $obj:expr, $exp:expr) => (
        #[test]
        fn $f() {
            assert_eq!(
                ::serde_urlencoded::to_string($obj),
                Ok($exp.to_string())
            )
        }
    )
}


/// Implements the `Serialize` trait for types that do not require
/// serialization. This provides a workaround for a limitation in
/// `serde_urlencoded`, that prevents unit structs from being serialized.
///
macro_rules! impl_skip_serialize {
    ($ty:ty) => (
        impl ::serde::Serialize for $ty {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::Serializer
            {
                serializer.serialize_none()
            }
        }
    )
}


mod add;
mod bitswap;
mod block;
mod bootstrap;
mod cat;
mod commands;
mod config;
mod dag;
mod ls;
mod object;
mod pin;
mod ping;
mod pubsub;
mod refs;
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
