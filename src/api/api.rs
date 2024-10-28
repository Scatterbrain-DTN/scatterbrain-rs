use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use bip39::Mnemonic;
use chrono::NaiveDateTime;
pub use flutter_rust_bridge::{frb, DartFnFuture};
use sodiumoxide::base64;
use sodiumoxide::crypto::generichash;
use sodiumoxide::crypto::kx::{client_session_keys, PublicKey, SessionKey};
pub use tokio::sync::RwLock;
use tokio::sync::RwLockWriteGuard;
use uuid::Uuid;

use super::error::Error;
use super::response::ToUuid;
use super::serialize::ProtoStream;
use super::types::DartFuture;
pub use super::types::DartSyncFuture;
pub use super::types::{CryptoConfig, ImportIdentityState};
use super::types::{PairingAck, PairingInitiate};
pub use super::{error::SbResult, mdns::HostRecord};
use crate::connection::SessionTrait;
pub use crate::crypto::SessionState;
use crate::crypto::{CryptoMessageWrapper, EncodeB64, Session};
use crate::proto::{PairingSynAck, SbEvent};
pub use crate::response::{Identity, Message};
use crate::types::{Ack, CryptoMessage, PairingRequest};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
type SbSessionInner = Arc<RwLock<dyn SessionTrait + Send + Sync>>;
pub struct SbSession(pub(crate) SbSessionInner);

impl Clone for SbSession {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

pub use async_trait::async_trait;

pub trait ProtoStreamTrait {
    fn try_pair_confirm(
        self: Box<Self>,
        session: TryPairConfirm,
        accept: bool,
    ) -> DartSyncFuture<'static, SbResult<SbSession>>;
}

pub struct PairingSession {
    pub coin: Vec<String>,
    pub(crate) state: SessionState,
    pub session: Uuid,
    pub(crate) tx: SessionKey,
    pub(crate) rx: SessionKey,
    pub(crate) remotekey: PublicKey,

    pub(crate) stream: Box<dyn ProtoStreamTrait + Send + Sync>,
}

pub struct TryPairConfirm {
    pub(crate) state: SessionState,
    pub(crate) session: Uuid,
    pub(crate) tx: SessionKey,
    pub(crate) rx: SessionKey,
    pub(crate) remotekey: PublicKey,
}

#[frb(non_opaque)]
pub struct PairResult {
    pub remotekey: String,
    pub session: SbSession,
}

impl PairingSession {
    pub async fn try_pair_confirm(self, accept: bool) -> anyhow::Result<PairResult> {
        let remotekey = base64::encode(&self.remotekey.0, base64::Variant::UrlSafe);
        let confirm = TryPairConfirm {
            state: self.state,
            session: self.session,
            tx: self.tx,
            rx: self.rx,
            remotekey: self.remotekey,
        };
        let s = self.stream.try_pair_confirm(confirm, accept).await?;
        s.on_connect().await;
        Ok(PairResult {
            session: s,
            remotekey,
        })
    }
}

#[frb(non_opaque)]
pub enum PairStatus {
    Paired(SbSession),
    NotPaired(PairingSession),
}

impl From<PairingSession> for TryPairConfirm {
    fn from(value: PairingSession) -> Self {
        TryPairConfirm {
            state: value.state,
            session: value.session,
            tx: value.tx,
            rx: value.rx,
            remotekey: value.remotekey,
        }
    }
}

impl HostRecord {
    pub async fn connect(&self, state: CryptoConfig) -> anyhow::Result<Option<SbSession>> {
        let proto = self.clone().connect_impl().await?;

        if let Some(session) = proto.key_exchange(SessionState::from_b64(state)?).await? {
            let sb_session = SbSession(Arc::new(RwLock::new(session)));
            sb_session.on_connect().await;
            Ok(Some(sb_session))
        } else {
            Ok(None)
        }
    }

    pub async fn try_pair(
        &self,
        state: CryptoConfig,
        app_name: String,
        on_connect: impl Fn(Option<SbSession>) -> DartFnFuture<()> + Send + Sync + 'static,
    ) -> anyhow::Result<PairStatus> {
        let mut proto = self.clone().connect_impl().await?;
        proto.on_connect = Some(Box::new(on_connect));
        let v = proto
            .try_pair(SessionState::from_b64(state)?, app_name)
            .await?;

        Ok(v)
    }

    pub async fn pair(
        &self,
        state: CryptoConfig,
        app_name: String,
        cb: impl FnOnce(Vec<String>) -> DartFnFuture<bool>,
    ) -> anyhow::Result<SbSession> {
        let proto = self.clone().connect_impl().await?;
        let session = proto
            .pair(SessionState::from_b64(state)?, app_name, |mn| async move {
                Ok(cb(mn.word_iter().map(|v| v.to_owned()).collect()).await)
            })
            .await?;

        Ok(SbSession(Arc::new(RwLock::new(session))))
    }
}

impl SbSession {
    pub async fn set_on_connect(
        &self,
        on_connect: impl Fn(Option<SbSession>) -> DartFnFuture<()> + Send + Sync + Sized + 'static,
    ) {
        self.0.write().await.set_on_connect(Box::new(on_connect))
    }

    pub async fn on_connect(&self) {
        if let Some(on_connect) = self.0.read().await.on_connect() {
            on_connect(Some(self.clone())).await;
        }
    }

    pub async fn get_identity(&self, id: Option<Uuid>) -> anyhow::Result<Vec<Identity>> {
        Ok(self.0.write().await.get_identity(id).await?)
    }

    pub async fn is_closed(&self) -> anyhow::Result<bool> {
        Ok(self.0.write().await.is_closed().await?)
    }

    pub async fn disconnect(&self) {
        if let Some(on_connect) = self.0.read().await.on_connect() {
            on_connect(None).await
        }
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
    fn set_on_connect(
        &mut self,
        on_connect: Box<dyn Fn(Option<SbSession>) -> DartFnFuture<()> + Send + Sync + 'static>,
    ) {
        self.deref_mut().set_on_connect(on_connect);
    }

    fn on_connect<'a>(
        &'a self,
    ) -> Option<&'a Box<dyn Fn(Option<SbSession>) -> DartFnFuture<()> + Send + Sync + 'static>>
    {
        self.deref().on_connect()
    }

    fn is_closed<'a>(&'a mut self) -> DartFuture<'a, SbResult<bool>> {
        Box::pin(async move { Ok(self.deref_mut().is_closed().await?) })
    }

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

impl<A> ProtoStreamTrait for ProtoStream<A>
where
    A: Unpin + Send + AsyncReadExt + AsyncWriteExt + Send + Sync + 'static,
{
    fn try_pair_confirm(
        mut self: Box<Self>,
        session: TryPairConfirm,
        accept: bool,
    ) -> DartSyncFuture<'static, SbResult<SbSession>> {
        Box::pin(async move {
            if accept {
                let mut ack = PairingSynAck::default();
                ack.success = true;
                self.write_message(CryptoMessageWrapper::new_message(&ack, &session.rx)?.message())
                    .await?;

                Ok(SbSession(Arc::new(RwLock::new(Session {
                    session: session.session,
                    rx: session.rx,
                    tx: session.tx,
                    state: SessionState {
                        secretkey: session.state.secretkey,
                        pubkey: session.state.pubkey,
                        remotekey: Some(session.remotekey),
                    },
                    stream: *self,
                }))))
            } else {
                let mut ack = PairingSynAck::default();
                ack.success = false;
                ack.message = "pairing request rejected".to_owned();
                self.write_message(CryptoMessageWrapper::new_message(&ack, &session.rx)?.message())
                    .await?;
                return Err(Error::PairingFailed);
            }
        })
    }
}

impl<A> ProtoStream<A>
where
    A: Unpin + Send + AsyncReadExt + AsyncWriteExt + Send + Sync + 'static,
{
    pub async fn try_pair(mut self, state: SessionState, app_name: String) -> SbResult<PairStatus> {
        let i = PairingInitiate {
            pubkey: state.pubkey.0.iter().copied().collect(),
        };
        self.write_message(&i).await?;
        let v: PairingAck = self.read_message().await?;

        let session_id = v
            .session
            .ok_or_else(|| Error::CorruptHeader)?
            .session
            .ok_or_else(|| Error::CorruptHeader)?;
        let sp = PublicKey(
            v.pubkey
                .try_into()
                .map_err(|_| Error::Crypto("pubkey wrong size".to_owned()))?,
        );

        let (rx, tx) = client_session_keys(&state.pubkey, &state.secretkey, &sp).unwrap();
        if let Some(remotekey) = state.remotekey {
            if remotekey.0 != sp.0 {
                return Err(Error::MitmDetected);
            }
            Ok(PairStatus::Paired(SbSession(Arc::new(RwLock::new(
                Session {
                    session: session_id.as_uuid(),
                    rx,
                    tx,
                    state: SessionState {
                        secretkey: state.secretkey,
                        pubkey: state.pubkey,
                        remotekey: Some(remotekey),
                    },
                    stream: self,
                },
            )))))
        } else {
            let mut pr = PairingRequest::default();
            pr.name = app_name;
            pr.session = v.session;
            let pr = CryptoMessageWrapper::new_message(&pr, &rx)?;
            self.write_message(pr.message()).await?;

            let fingerprint =
                generichash::hash(&i.pubkey, Some(generichash::DIGEST_MIN), None).unwrap();
            let words = Mnemonic::from_entropy(fingerprint.as_ref())?;

            let v: CryptoMessage = self.read_message().await?;

            let v = CryptoMessageWrapper::new(v);

            let v: Ack = v.decrypt(&tx)?;

            log::info!("got ack {}", v.success);

            if !v.success {
                return Err(Error::PairingFailed);
            }

            Ok(PairStatus::NotPaired(PairingSession {
                session: session_id.as_uuid(),
                rx,
                tx,
                state: SessionState {
                    secretkey: state.secretkey,
                    pubkey: state.pubkey,
                    remotekey: Some(sp),
                },
                coin: words.word_iter().map(|v| v.to_owned()).collect(),
                remotekey: sp,
                stream: Box::new(self),
            }))
        }
    }
}
