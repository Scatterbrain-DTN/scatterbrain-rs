#[cfg(feature = "flutter")]
mod frb_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
use error::{Error, SbResult};

pub mod api;
pub mod connection;
pub mod constants;
pub mod crypto;
pub use api::error;
pub use api::mdns;
pub use api::response;
pub use api::serialize;
pub use api::types;
pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/proto.rs"));
}
pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
pub fn init() -> SbResult<()> {
    sodiumoxide::init().map_err(|_| Error::Crypto("Failed to init".to_owned()))?;
    Ok(())
}
