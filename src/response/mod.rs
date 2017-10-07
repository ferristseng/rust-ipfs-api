pub use self::add::*;
pub use self::bitswap::*;
pub use self::block::*;
pub use self::bootstrap::*;
pub use self::cat::*;
pub use self::commands::*;
pub use self::config::*;
pub use self::dag::*;
pub use self::dht::*;
pub use self::diag::*;
pub use self::dns::*;
pub use self::file::*;
pub use self::files::*;
pub use self::filestore::*;
pub use self::id::*;
pub use self::key::*;
pub use self::log::*;
pub use self::mount::*;
pub use self::name::*;
pub use self::object::*;


/// Create a test to deserialize a file to the given instance.
///
#[cfg(test)]
macro_rules! deserialize_test {
    ($f:ident, $ty:ty) => (
        #[test]
        fn $f() {
            let raw = include_str!(concat!("tests/", stringify!($f), ".json"));

            match ::serde_json::from_str::<$ty>(raw) {
                Ok(_) => assert!(true),
                Err(e) => assert!(false, format!("failed with error: {}", e))
            };
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
mod dht;
mod diag;
mod dns;
mod file;
mod files;
mod filestore;
mod get;
mod id;
mod key;
mod log;
mod mount;
mod name;
mod object;


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IpfsFile {
    pub hash: String,
    pub size: u64,

    #[serde(rename = "Type")]
    pub typ: String,

    #[serde(default)]
    pub links: Vec<IpfsFileLink>,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IpfsFileLink {
    pub name: String,
    pub hash: String,
    pub size: u64,

    #[serde(rename = "Type")]
    pub typ: String,
}
