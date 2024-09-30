use bip39::Mnemonic;
use chrono::NaiveDateTime;
use sodiumoxide::crypto::{
    generichash::{self},
    kx::{client_session_keys, PublicKey},
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;

pub use crate::types::SessionTrait;
pub use crate::{
    crypto::{CryptoMessageWrapper, Session, SessionState},
    error::{Error, IntoRemoteErr, SbResult},
    mdns::HostRecord,
    proto::{
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
    response::{Identity, Message},
    serialize::{ProtoStream, ToUuid},
    types::ImportIdentityState,
};
use crate::{
    proto::PairingSynAck,
    types::{DartFuture, PairingSession},
};

pub use std::{future::Future, net::SocketAddr};
pub use tokio::net::TcpStream;

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

impl<A> SessionTrait for Session<A>
where
    A: Unpin + Send + AsyncReadExt + AsyncWriteExt + Sync,
    Self: Sized,
{
    fn get_identity<'a>(&'a mut self, id: Option<Uuid>) -> DartFuture<'a, SbResult<Vec<Identity>>> {
        Box::pin(async move {
            if let Option::Some(__ret) = Option::None::<SbResult<Vec<Identity>>> {
                #[allow(unreachable_code)]
                return __ret;
            }
            let mut __self = self;
            let id = id;
            let __ret: SbResult<Vec<Identity>> = {
                let cmd = GetIdentityCommand {
                    header: Some(__self.get_header()),
                    id: id.map(|v| Id::Identity(v.as_proto())),
                    owned: false,
                };
                __self.write_crypto(cmd).await?;
                let id: IdentityResponse = __self.read_crypto().await?;
                id.try_into()
            };
            #[allow(unreachable_code)]
            __ret
        })
    }

    fn get_events<'a>(
        &'a mut self,
        block: bool,
        count: Option<u32>,
    ) -> DartFuture<'a, SbResult<Vec<SbEvent>>> {
        Box::pin(async move {
            if let Option::Some(__ret) = Option::None::<SbResult<Vec<SbEvent>>> {
                #[allow(unreachable_code)]
                return __ret;
            }
            let mut __self = self;
            let block = block;
            let count = count;
            let __ret: SbResult<Vec<SbEvent>> = {
                let cmd = GetEvents {
                    header: Some(__self.get_header()),
                    block,
                    maybe_count: count.map(|v| MaybeCount::Count(v)),
                };
                __self.write_crypto(cmd).await?;
                let resp: SbEvents = __self.read_crypto().await?;
                Ok(resp.events)
            };
            #[allow(unreachable_code)]
            __ret
        })
    }

    fn get_messages<'a>(
        &'a mut self,
        application: String,
        limit: Option<i32>,
    ) -> DartFuture<'a, SbResult<Vec<Message>>> {
        Box::pin(async move {
            if let Option::Some(__ret) = Option::None::<SbResult<Vec<Message>>> {
                #[allow(unreachable_code)]
                return __ret;
            }
            let mut __self = self;
            let application = application;
            let limit = limit;
            let __ret: SbResult<Vec<Message>> = {
                let cmd = GetMessagesCmd {
                    header: Some(__self.get_header()),
                    time_slice: None,
                    maybe_application: Some(MaybeApplication::Application(application)),
                    limit: limit.unwrap_or(-1),
                };
                __self.write_crypto(cmd).await?;
                let m: MessageResponse = __self.read_crypto().await?;
                m.try_into()
            };
            #[allow(unreachable_code)]
            __ret
        })
    }

    fn send_messages<'a>(
        &'a mut self,
        messages: Vec<Message>,
        sign_identity: Option<Uuid>,
    ) -> DartFuture<'a, SbResult<()>> {
        Box::pin(async move {
            if let Option::Some(__ret) = Option::None::<SbResult<()>> {
                #[allow(unreachable_code)]
                return __ret;
            }
            let mut __self = self;
            let messages = messages;
            let sign_identity = sign_identity;
            let __ret: SbResult<()> = {
                let cmd = SendMessageCmd {
                    header: Some(__self.get_header()),
                    messages: messages.into_iter().map(|v| v.into()).collect(),
                    sign_identity: sign_identity.map(|v| SignIdentity::Identity(v.as_proto())),
                };
                __self.write_crypto(cmd).await?;
                let m: UnitResponse = __self.read_crypto().await?;
                m.into_remote_err()?;
                Ok(())
            };
            #[allow(unreachable_code)]
            __ret
        })
    }

    fn initiate_identity_import<'a>(
        &'a mut self,
        id: Option<Uuid>,
    ) -> DartFuture<'a, SbResult<ImportIdentityState>> {
        Box::pin(async move {
            if let Option::Some(__ret) = Option::None::<SbResult<ImportIdentityState>> {
                #[allow(unreachable_code)]
                return __ret;
            }
            let mut __self = self;
            let id = id;
            let __ret: SbResult<ImportIdentityState> = {
                let cmd = ImportIdentityCommand {
                    header: Some(__self.get_header()),
                    maybe_handle: id.map(|v| MaybeHandle::Handle(v.as_proto())),
                };
                __self.write_crypto(cmd).await?;
                let resp: ImportIdentityResponse = __self.read_crypto().await?;
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
            };
            #[allow(unreachable_code)]
            __ret
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
            if let Option::Some(__ret) = Option::None::<SbResult<Vec<Message>>> {
                #[allow(unreachable_code)]
                return __ret;
            }
            let mut __self = self;
            let application = application;
            let limit = limit;
            let start_date = start_date;
            let end_date = end_date;
            let __ret: SbResult<Vec<Message>> = {
                let cmd = GetMessagesCmd {
                    header: Some(__self.get_header()),
                    time_slice: Some(TimeSlice::SendDate(TimeRange {
                        start: start_date.and_utc().timestamp(),
                        end: end_date.and_utc().timestamp(),
                    })),
                    maybe_application: Some(MaybeApplication::Application(application)),
                    limit: limit.unwrap_or(-1),
                };
                __self.write_crypto(cmd).await?;
                let m: MessageResponse = __self.read_crypto().await?;
                m.try_into()
            };
            #[allow(unreachable_code)]
            __ret
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
            if let Option::Some(__ret) = Option::None::<SbResult<Vec<Message>>> {
                #[allow(unreachable_code)]
                return __ret;
            }
            let mut __self = self;
            let application = application;
            let limit = limit;
            let start_date = start_date;
            let end_date = end_date;
            let __ret: SbResult<Vec<Message>> = {
                let cmd = GetMessagesCmd {
                    header: Some(__self.get_header()),
                    time_slice: Some(TimeSlice::ReceiveDate(TimeRange {
                        start: start_date.and_utc().timestamp(),
                        end: end_date.and_utc().timestamp(),
                    })),
                    maybe_application: Some(MaybeApplication::Application(application)),
                    limit: limit.unwrap_or(-1),
                };
                __self.write_crypto(cmd).await?;
                let m: MessageResponse = __self.read_crypto().await?;
                m.try_into()
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}

impl<A> ProtoStream<A>
where
    A: Unpin + Send + AsyncReadExt + AsyncWriteExt,
{
    pub async fn key_exchange(mut self, state: SessionState) -> SbResult<Option<Session<A>>> {
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
            Ok(Some(Session {
                session: session_id.as_uuid(),
                rx,
                tx,
                state: SessionState {
                    secretkey: state.secretkey,
                    pubkey: state.pubkey,
                    remotekey: Some(remotekey),
                },
                stream: self,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn try_pair<F, Fut>(
        &mut self,
        state: SessionState,
        app_name: String,
    ) -> SbResult<Option<PairingSession>> {
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
            Ok(None)
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

            Ok(Some(PairingSession {
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
            }))
        }
    }

    pub async fn try_pair_confirm(
        mut self,
        session: PairingSession,
        accept: bool,
    ) -> SbResult<Session<A>> {
        if accept {
            let mut ack = PairingSynAck::default();
            ack.success = true;
            self.write_message(CryptoMessageWrapper::new_message(&ack, &session.rx)?.message())
                .await?;

            Ok(Session {
                session: session.session,
                rx: session.rx,
                tx: session.tx,
                state: SessionState {
                    secretkey: session.state.secretkey,
                    pubkey: session.state.pubkey,
                    remotekey: Some(session.remotekey),
                },
                stream: self,
            })
        } else {
            let mut ack = PairingSynAck::default();
            ack.success = false;
            ack.message = "pairing request rejected".to_owned();
            self.write_message(CryptoMessageWrapper::new_message(&ack, &session.rx)?.message())
                .await?;
            return Err(Error::PairingFailed);
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
            Ok(Session {
                session: session_id.as_uuid(),
                rx,
                tx,
                state: SessionState {
                    secretkey: state.secretkey,
                    pubkey: state.pubkey,
                    remotekey: Some(remotekey),
                },
                stream: self,
            })
        } else {
            let mut pr = PairingRequest::default();
            pr.name = app_name;
            pr.session = v.session;
            let pr = CryptoMessageWrapper::new_message(&pr, &rx)?;
            self.write_message(pr.message()).await?;

            let fingerprint =
                generichash::hash(&i.pubkey, Some(generichash::DIGEST_MIN), None).unwrap();
            let words = Mnemonic::from_entropy(fingerprint.as_ref())?;
            let confirmed = cb(words).await?; // I hate HRTBs

            let v: CryptoMessage = self.read_message().await?;

            let v = CryptoMessageWrapper::new(v);

            let ack: Ack = v.decrypt(&tx)?;

            log::info!("got ack {}", ack.success);

            let mut synack = PairingSynAck::default();
            if !ack.success || !confirmed {
                synack.success = false;
                synack.message = "Pairing request rejected".to_owned();
                self.write_message(CryptoMessageWrapper::new_message(&ack, &rx)?.message())
                    .await?;
                return Err(Error::PairingFailed);
            }

            synack.success = true;
            self.write_message(CryptoMessageWrapper::new_message(&synack, &rx)?.message())
                .await?;

            Ok(Session {
                session: session_id.as_uuid(),
                rx,
                tx,
                state: SessionState {
                    secretkey: state.secretkey,
                    pubkey: state.pubkey,
                    remotekey: Some(sp),
                },
                stream: self,
            })
        }
    }
}
