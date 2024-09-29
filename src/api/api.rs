use std::ops::DerefMut;
use std::sync::Arc;

use chrono::NaiveDateTime;
use flutter_rust_bridge::DartFnFuture;
pub use tokio::sync::RwLock;
use tokio::sync::RwLockWriteGuard;
use uuid::Uuid;

pub use super::types::DartFuture;
pub use super::types::{CryptoConfig, ImportIdentityState, SessionTrait};
pub use super::{error::SbResult, mdns::HostRecord};
use crate::crypto::EncodeB64;
pub use crate::crypto::SessionState;
use crate::proto::SbEvent;
pub use crate::response::{Identity, Message};
type SbSessionInner = Arc<RwLock<dyn SessionTrait + Send + Sync>>;
pub struct SbSession(SbSessionInner);
pub use async_trait::async_trait;

impl HostRecord {
    pub async fn connect(self, state: CryptoConfig) -> anyhow::Result<Option<SbSession>> {
        let proto = self.connect_impl().await?;

        if let Some(session) = proto.key_exchange(SessionState::from_b64(state)?).await? {
            Ok(Some(SbSession(Arc::new(RwLock::new(session)))))
        } else {
            Ok(None)
        }
    }

    pub async fn pair(
        self,
        state: CryptoConfig,
        app_name: String,
        cb: impl FnOnce(Vec<String>) -> DartFnFuture<bool>,
    ) -> anyhow::Result<SbSession> {
        let proto = self.connect_impl().await?;
        let session = proto
            .pair(SessionState::from_b64(state)?, app_name, |mn| async move {
                Ok(cb(mn.word_iter().map(|v| v.to_owned()).collect()).await)
            })
            .await?;

        Ok(SbSession(Arc::new(RwLock::new(session))))
    }
}

impl SbSession {
    pub async fn get_identity(&self, id: Option<Uuid>) -> anyhow::Result<Vec<Identity>> {
        Ok(self.0.write().await.get_identity(id).await?)
    }

    pub async fn get_events(
        &self,
        block: bool,
        count: Option<u32>,
    ) -> anyhow::Result<Vec<SbEvent>> {
        Ok(self.0.write().await.get_events(block, count).await?)
    }

    pub async fn get_messages<'a>(
        &self,
        application: String,
        limit: Option<i32>,
    ) -> anyhow::Result<Vec<Message>> {
        Ok(self
            .0
            .write()
            .await
            .get_messages(application, limit)
            .await?)
    }

    pub async fn send_messages<'a>(
        &self,
        messages: Vec<Message>,
        sign_identity: Option<Uuid>,
    ) -> anyhow::Result<()> {
        Ok(self
            .0
            .write()
            .await
            .send_messages(messages, sign_identity)
            .await?)
    }

    pub async fn initiate_identity_import<'a>(
        &'a self,
        id: Option<Uuid>,
    ) -> anyhow::Result<ImportIdentityState> {
        Ok(self.0.write().await.initiate_identity_import(id).await?)
    }

    pub async fn get_messages_send_date<'a>(
        &'a self,
        application: String,
        limit: Option<i32>,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
    ) -> anyhow::Result<Vec<Message>> {
        Ok(self
            .0
            .write()
            .await
            .get_messages_send_date(application, limit, start_date, end_date)
            .await?)
    }

    pub async fn get_messages_recieve_date<'a>(
        &'a self,
        application: String,
        limit: Option<i32>,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
    ) -> anyhow::Result<Vec<Message>> {
        Ok(self
            .0
            .write()
            .await
            .get_messages_recieve_date(application, limit, start_date, end_date)
            .await?)
    }
}

impl<'b> SessionTrait for RwLockWriteGuard<'b, dyn SessionTrait + Send + Sync>
where
    Self: Sized,
{
    fn get_identity<'a>(&'a mut self, id: Option<Uuid>) -> DartFuture<'a, SbResult<Vec<Identity>>> {
        Box::pin(async move {
            let id = self.deref_mut().get_identity(id).await?;
            Ok(id)
        })
    }

    fn get_events<'a>(
        &'a mut self,
        block: bool,
        count: Option<u32>,
    ) -> DartFuture<'a, SbResult<Vec<SbEvent>>> {
        Box::pin(async move {
            let id = self.deref_mut().get_events(block, count).await?;
            Ok(id)
        })
    }

    fn get_messages<'a>(
        &'a mut self,
        application: String,
        limit: Option<i32>,
    ) -> DartFuture<'a, SbResult<Vec<Message>>> {
        Box::pin(async move {
            let id = self.deref_mut().get_messages(application, limit).await?;
            Ok(id)
        })
    }

    fn send_messages<'a>(
        &'a mut self,
        messages: Vec<Message>,
        sign_identity: Option<Uuid>,
    ) -> DartFuture<'a, SbResult<()>> {
        Box::pin(async move {
            let id = self
                .deref_mut()
                .send_messages(messages, sign_identity)
                .await?;
            Ok(id)
        })
    }

    fn initiate_identity_import<'a>(
        &'a mut self,
        id: Option<Uuid>,
    ) -> DartFuture<'a, SbResult<ImportIdentityState>> {
        Box::pin(async move {
            let id = self.deref_mut().initiate_identity_import(id).await?;
            Ok(id)
        })
    }

    fn get_messages_send_date<'a>(
        &'a mut self,
        application: String,
        limit: Option<i32>,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
    ) -> DartFuture<'a, SbResult<Vec<Message>>> {
        Box::pin(async move {
            let id = self
                .deref_mut()
                .get_messages_send_date(application, limit, start_date, end_date)
                .await?;
            Ok(id)
        })
    }

    fn get_messages_recieve_date<'a>(
        &'a mut self,
        application: String,
        limit: Option<i32>,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
    ) -> DartFuture<'a, SbResult<Vec<Message>>> {
        Box::pin(async move {
            let id = self
                .deref_mut()
                .get_messages_recieve_date(application, limit, start_date, end_date)
                .await?;
            Ok(id)
        })
    }
}
