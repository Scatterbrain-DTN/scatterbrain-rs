pub use crate::{
    api::proto::{
        get_events::MaybeCount,
        get_identity_command::Id,
        get_messages_cmd::{MaybeApplication, TimeRange, TimeSlice},
        import_identity_command::MaybeHandle,
        import_identity_response::{FinalResponse, State},
        send_message_cmd::SignIdentity,
        Ack, CryptoMessage, GetEvents, GetIdentityCommand, GetMessagesCmd, IdentityResponse,
        ImportIdentityCommand, ImportIdentityResponse, MessageResponse, PairingAck,
        PairingInitiate, PairingRequest, SbEvent, SbEvents, SendMessageCmd, UnitResponse,
    },
    crypto::{CryptoMessageWrapper, Session, SessionState},
    error::{Error, IntoRemoteErr, SbResult},
    mdns::HostRecord,
    response::{Identity, Message},
    serialize::{ProtoStream, ToUuid},
    types::ImportIdentityState,
};
use crate::{
    api::{
        proto::get_messages_cmd::time_range::{EndPoint, StartPoint},
        proto::PairingSynAck,
    },
    types::DartFuture,
};
use chrono::NaiveDateTime;
use dryoc::generichash::Key;

use bip39::Mnemonic;
use dryoc::{
    constants::CRYPTO_GENERICHASH_BYTES_MIN, generichash::GenericHash, kx::PublicKey,
    types::StackByteArray,
};
pub use std::{future::Future, net::SocketAddr};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
pub use tokio::net::TcpStream;
use uuid::Uuid;

impl From<SocketAddr> for HostRecord {
    fn from(value: SocketAddr) -> Self {
        Self {
            name: value.to_string(),
            addr: [value.ip()].into_iter().collect(),
            port: value.port(),
        }
    }
}

impl HostRecord {
    #[cfg(not(feature = "flutter"))]
    pub async fn connect(self) -> SbResult<ProtoStream<TcpStream>> {
        self.connect_impl().await
    }

    pub(crate) async fn connect_impl(self) -> SbResult<ProtoStream<TcpStream>> {
        for addr in self.addr {
            println!("attempting to connect to {}", addr);
            match TcpStream::connect((addr, self.port)).await {
                Ok(c) => return Ok(ProtoStream::new(c)),
                Err(err) => log::warn!("Failed to connect to {}: {}", addr, err),
            }
        }
        Err(Error::NoAddr)
    }
}

impl<A> ProtoStream<A>
where
    A: Unpin + Send + AsyncReadExt + AsyncWriteExt + 'static,
{
    pub async fn key_exchange(mut self, state: SessionState) -> SbResult<Option<Session<A>>> {
        let i = PairingInitiate {
            pubkey: state.kp.public_key.iter().copied().collect(),
        };
        self.write_message(&i).await?;

        let v: PairingAck = self.read_message().await?;
        let session_id = v
            .session
            .ok_or_else(|| Error::CorruptHeader)?
            .session
            .ok_or_else(|| Error::CorruptHeader)?;

        let ack_remote_key: PublicKey = v.pubkey.as_slice().try_into()?;
        let s = dryoc::kx::Session::new_client(&state.kp, &ack_remote_key)?;

        log::debug!("tx key {:?}", s.tx_as_slice());
        log::debug!("rx key {:?}", s.rx_as_slice());

        if let Some(remotekey) = state.remotekey {
            if remotekey != ack_remote_key {
                return Err(Error::MitmDetected);
            }
            Ok(Some(Session {
                session: session_id.as_uuid(),
                session_keys: s,
                state: SessionState {
                    kp: state.kp,
                    remotekey: Some(remotekey),
                },
                stream: self,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn pair<F, Fut>(
        mut self,
        state: SessionState,
        app_name: String,
        cb: F,
    ) -> SbResult<Session<A>>
    where
        F: FnOnce(Mnemonic) -> Fut,
        Fut: Future<Output = std::result::Result<bool, Box<dyn std::error::Error + Send + Sync>>>,
    {
        let i = PairingInitiate {
            pubkey: state.kp.public_key.iter().copied().collect(),
        };
        self.write_message(&i).await?;

        let v: PairingAck = self.read_message().await?;
        let session_id = v
            .session
            .ok_or_else(|| Error::CorruptHeader)?
            .session
            .ok_or_else(|| Error::CorruptHeader)?;

        let ack_remote_key: PublicKey = v.pubkey.as_slice().try_into()?;

        let s = dryoc::kx::Session::new_client(&state.kp, &ack_remote_key)?;

        log::debug!("tx key {:?}", s.tx_as_slice());
        log::debug!("rx key {:?}", s.rx_as_slice());

        if let Some(remotekey) = state.remotekey {
            if remotekey != ack_remote_key {
                return Err(Error::MitmDetected);
            }
            Ok(Session {
                session: session_id.as_uuid(),
                session_keys: s,
                state: SessionState {
                    kp: state.kp,
                    remotekey: Some(remotekey),
                },
                stream: self,
            })
        } else {
            let mut pr = PairingRequest::default();
            pr.name = app_name;
            pr.session = v.session;
            let pr = CryptoMessageWrapper::new_message(&pr, s.rx_as_array())?;
            self.write_message(pr.message()).await?;
            let fingerprint: StackByteArray<{ CRYPTO_GENERICHASH_BYTES_MIN }> =
                GenericHash::hash(&i.pubkey, None::<&Key>).unwrap();
            let words = Mnemonic::from_entropy(fingerprint.as_ref())?;
            let confirmed = cb(words).await?; // I hate HRTBs

            let v: CryptoMessage = self.read_message().await?;

            let v = CryptoMessageWrapper::new(v);

            let ack: Ack = v.decrypt(s.tx_as_array())?;

            log::info!("got ack {}", ack.success);

            let mut synack = PairingSynAck::default();
            if !ack.success || !confirmed {
                synack.success = false;
                synack.message = "Pairing request rejected".to_owned();
                self.write_message(
                    CryptoMessageWrapper::new_message(&ack, s.rx_as_array())?.message(),
                )
                .await?;
                return Err(Error::PairingFailed);
            }

            synack.success = true;
            self.write_message(
                CryptoMessageWrapper::new_message(&synack, s.rx_as_array())?.message(),
            )
            .await?;

            Ok(Session {
                session: session_id.as_uuid(),
                session_keys: s,
                state: SessionState {
                    kp: state.kp,
                    remotekey: Some(ack_remote_key),
                },
                stream: self,
            })
        }
    }
}

impl<A> SessionTrait for Session<A>
where
    A: Unpin + Send + AsyncReadExt + AsyncWriteExt,
{
    #[cfg(feature = "flutter")]
    fn set_on_connect(
        &mut self,
        on_connect: Box<
            dyn Fn(Option<crate::api::api::SbSession>) -> flutter_rust_bridge::DartFnFuture<()>
                + Send
                + Sync
                + 'static,
        >,
    ) {
        self.stream.on_connect = Some(Box::new(on_connect));
    }

    #[cfg(feature = "flutter")]
    fn on_connect<'a>(
        &'a self,
    ) -> Option<
        &'a Box<
            dyn Fn(Option<crate::api::api::SbSession>) -> flutter_rust_bridge::DartFnFuture<()>
                + Send
                + Sync
                + 'static,
        >,
    > {
        self.stream.on_connect.as_ref()
    }

    fn get_identity<'a>(&'a mut self, id: Option<Uuid>) -> DartFuture<'a, SbResult<Vec<Identity>>> {
        Box::pin(async move {
            let cmd = GetIdentityCommand {
                header: Some(self.get_header()),
                id: id.map(|v| Id::Identity(v.as_proto())),
                owned: false,
            };
            self.write_crypto(cmd).await?;
            let id: IdentityResponse = self.read_crypto().await?;
            id.try_into()
        })
    }

    fn is_closed<'a>(&'a mut self) -> DartFuture<'a, SbResult<bool>> {
        Box::pin(async move { Ok(self.is_disconnected()) })
    }

    fn get_events<'a>(
        &'a mut self,
        block: bool,
        count: Option<u32>,
    ) -> DartFuture<'a, SbResult<Vec<SbEvent>>> {
        Box::pin(async move {
            let cmd = GetEvents {
                header: Some(self.get_header()),
                block,
                maybe_count: count.map(|v| MaybeCount::Count(v)),
            };
            self.write_crypto(cmd).await?;
            let resp: SbEvents = self.read_crypto().await?;
            Ok(resp.events)
        })
    }

    fn get_messages<'a>(
        &'a mut self,
        application: String,
        limit: Option<i32>,
    ) -> DartFuture<'a, SbResult<Vec<Message>>> {
        Box::pin(async move {
            let cmd = GetMessagesCmd {
                header: Some(self.get_header()),
                time_slice: None,
                maybe_application: Some(MaybeApplication::Application(application)),
                limit: limit.unwrap_or(-1),
            };
            self.write_crypto(cmd).await?;
            let m: MessageResponse = self.read_crypto().await?;
            m.try_into()
        })
    }

    fn send_messages<'a>(
        &'a mut self,
        messages: Vec<Message>,
        sign_identity: Option<Uuid>,
    ) -> DartFuture<'a, SbResult<()>> {
        Box::pin(async move {
            let cmd = SendMessageCmd {
                header: Some(self.get_header()),
                messages: messages.into_iter().map(|v| v.into()).collect(),
                sign_identity: sign_identity.map(|v| SignIdentity::Identity(v.as_proto())),
            };
            self.write_crypto(cmd).await?;
            let m: UnitResponse = self.read_crypto().await?;
            m.into_remote_err()?;
            Ok(())
        })
    }

    fn initiate_identity_import<'a>(
        &'a mut self,
        id: Option<Uuid>,
    ) -> DartFuture<'a, SbResult<ImportIdentityState>> {
        Box::pin(async move {
            let cmd = ImportIdentityCommand {
                header: Some(self.get_header()),
                maybe_handle: id.map(|v| MaybeHandle::Handle(v.as_proto())),
            };
            self.write_crypto(cmd).await?;
            let resp: ImportIdentityResponse = self.read_crypto().await?;
            let state = resp
                .state
                .ok_or_else(|| Error::RemoteError("Missing state field".to_owned()))?;
            let res = match state {
                State::Handle(uuid) => ImportIdentityState::Initiated(uuid.as_uuid()),
                State::Final(FinalResponse { identity, .. }) => ImportIdentityState::Complete(
                    identity
                        .ok_or_else(|| Error::RemoteError("missing identity uuid".to_owned()))?
                        .as_uuid(),
                ),
            };
            Ok(res)
        })
    }

    fn get_messages_send_date<'a>(
        &'a mut self,
        application: String,
        limit: Option<i32>,
        start_date: Option<NaiveDateTime>,
        end_date: Option<NaiveDateTime>,
    ) -> DartFuture<'a, SbResult<Vec<Message>>> {
        Box::pin(async move {
            let cmd = GetMessagesCmd {
                header: Some(self.get_header()),
                time_slice: Some(TimeSlice::SendDate(TimeRange {
                    start_point: start_date.map(|v| StartPoint::Start(v.and_utc().timestamp())),
                    end_point: end_date.map(|v| EndPoint::End(v.and_utc().timestamp())),
                })),
                maybe_application: Some(MaybeApplication::Application(application)),
                limit: limit.unwrap_or(-1),
            };
            self.write_crypto(cmd).await?;
            let m: MessageResponse = self.read_crypto().await?;
            m.try_into()
        })
    }

    fn get_messages_recieve_date<'a>(
        &'a mut self,
        application: String,
        limit: Option<i32>,
        start_date: Option<NaiveDateTime>,
        end_date: Option<NaiveDateTime>,
    ) -> DartFuture<'a, SbResult<Vec<Message>>> {
        Box::pin(async move {
            let cmd = GetMessagesCmd {
                header: Some(self.get_header()),
                time_slice: Some(TimeSlice::SendDate(TimeRange {
                    start_point: start_date.map(|v| StartPoint::Start(v.and_utc().timestamp())),
                    end_point: end_date.map(|v| EndPoint::End(v.and_utc().timestamp())),
                })),
                maybe_application: Some(MaybeApplication::Application(application)),
                limit: limit.unwrap_or(-1),
            };
            self.write_crypto(cmd).await?;
            let m: MessageResponse = self.read_crypto().await?;
            m.try_into()
        })
    }
}

pub trait SessionTrait {
    fn get_identity<'a>(&'a mut self, id: Option<Uuid>) -> DartFuture<'a, SbResult<Vec<Identity>>>;
    #[cfg(feature = "flutter")]

    fn set_on_connect(
        &mut self,
        on_connect: Box<
            dyn Fn(Option<crate::api::api::SbSession>) -> flutter_rust_bridge::DartFnFuture<()>
                + Send
                + Sync
                + 'static,
        >,
    );

    #[cfg(feature = "flutter")]
    fn on_connect<'a>(
        &'a self,
    ) -> Option<
        &'a Box<
            dyn Fn(Option<crate::api::api::SbSession>) -> flutter_rust_bridge::DartFnFuture<()>
                + Send
                + Sync
                + 'static,
        >,
    >;

    fn get_events<'a>(
        &'a mut self,
        block: bool,
        count: Option<u32>,
    ) -> DartFuture<'a, SbResult<Vec<SbEvent>>>;

    fn get_messages<'a>(
        &'a mut self,
        application: String,
        limit: Option<i32>,
    ) -> DartFuture<'a, SbResult<Vec<Message>>>;

    fn send_messages<'a>(
        &'a mut self,
        messages: Vec<Message>,
        sign_identity: Option<Uuid>,
    ) -> DartFuture<'a, SbResult<()>>;

    fn initiate_identity_import<'a>(
        &'a mut self,
        id: Option<Uuid>,
    ) -> DartFuture<'a, SbResult<ImportIdentityState>>;

    fn get_messages_send_date<'a>(
        &'a mut self,
        application: String,
        limit: Option<i32>,
        start_date: Option<NaiveDateTime>,
        end_date: Option<NaiveDateTime>,
    ) -> DartFuture<'a, SbResult<Vec<Message>>>;

    fn get_messages_recieve_date<'a>(
        &'a mut self,
        application: String,
        limit: Option<i32>,
        start_date: Option<NaiveDateTime>,
        end_date: Option<NaiveDateTime>,
    ) -> DartFuture<'a, SbResult<Vec<Message>>>;

    fn is_closed<'a>(&'a mut self) -> DartFuture<'a, SbResult<bool>>;
}
