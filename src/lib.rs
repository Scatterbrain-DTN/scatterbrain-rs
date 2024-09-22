mod frb_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
use error::{Error, SbResult};

pub mod api;
pub mod constants;
pub mod crypto;
pub use api::connection;
pub use api::error;
pub use api::mdns;
pub use api::response;
pub use api::serialize;
pub mod types;

pub fn init() -> SbResult<()> {
    sodiumoxide::init().map_err(|_| Error::Crypto("Failed to init".to_owned()))?;
    Ok(())
}

mod proto {
    include!(concat!(env!("OUT_DIR"), "/proto.rs"));
}
