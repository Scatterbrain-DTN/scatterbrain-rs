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

#[cfg(feature = "flutter")]
pub mod frb {
    pub(crate) const FLUTTER_RUST_BRIDGE_CODEGEN_VERSION: &str = "2.7.0";
    flutter_rust_bridge::frb_generated_default_handler!();
}
