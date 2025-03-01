#[cfg(feature = "flutter")]
pub mod api;
pub mod error;
pub mod mdns;
#[cfg(feature = "flutter")]
pub mod mirror;
pub mod response;
pub mod serialize;
pub mod types;
pub use crate::types::GetType;

#[cfg_attr(feature = "flutter", flutter_rust_bridge::frb(ignore))]
pub mod proto {
    pub use crate::types::GetType;

    include!(concat!(env!("OUT_DIR"), "/proto.rs"));
}
