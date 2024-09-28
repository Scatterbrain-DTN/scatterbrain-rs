use std::sync::Arc;

use chrono::NaiveDateTime;
use flutter_rust_bridge::frb;
pub use tokio::sync::RwLock;
use uuid::Uuid;

pub use super::types::{B64SessionState, ImportIdentityState, SessionTrait};
pub use super::{error::SbResult, mdns::HostRecord};
use crate::crypto::EncodeB64;
pub use crate::crypto::SessionState;
use crate::proto::SbEvent;
pub use crate::response::{Identity, Message};
pub struct SbSession(Arc<RwLock<dyn SessionTrait + Send + Sync>>);
pub use async_trait::async_trait;

impl HostRecord {
    pub async fn connect(self, state: B64SessionState) -> anyhow::Result<Option<SbSession>> {
        let proto = self.connect_impl().await?;

        if let Some(session) = proto.key_exchange(SessionState::from_b64(state)?).await? {
            Ok(Some(SbSession(Arc::new(RwLock::new(session)))))
        } else {
            Ok(None)
        }
    }
}
