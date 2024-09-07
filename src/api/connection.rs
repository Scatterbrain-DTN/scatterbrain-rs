use bip39::Mnemonic;
use chrono::NaiveDateTime;
use sodiumoxide::crypto::{
    generichash::{self},
    kx::{client_session_keys, PublicKey},
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;

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
        Ack, CryptoMessage, Event, Events, GetEvents, GetIdentityCommand, GetMessagesCmd,
        IdentityResponse, ImportIdentityCommand, ImportIdentityResponse, MessageResponse,
        PairingAck, PairingInitiate, PairingRequest, SendMessageCmd, UnitResponse,
    },
    response::{Identity, Message},
    serialize::{ProtoStream, ToUuid},
    types::ImportIdentityState,
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
    pub async fn connect(self) -> SbResult<ProtoStream<TcpStream>> {
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

impl<A> Session<A>
where
    A: Unpin + Send + AsyncReadExt + AsyncWriteExt,
{
    pub async fn get_identity(&mut self, id: Option<Uuid>) -> SbResult<Vec<Identity>> {
        let cmd = GetIdentityCommand {
            header: Some(self.get_header()),
            id: id.map(|v| Id::Identity(v.as_proto())),
            owned: false,
        };

        self.write_crypto(cmd).await?;

        let id: IdentityResponse = self.read_crypto().await?;
        id.try_into()
    }

    pub async fn get_events(&mut self, block: bool, count: Option<u32>) -> SbResult<Vec<Event>> {
        let cmd = GetEvents {
            header: Some(self.get_header()),
            block,
            maybe_count: count.map(|v| MaybeCount::Count(v)),
        };

        self.write_crypto(cmd).await?;
        let resp: Events = self.read_crypto().await?;
        Ok(resp.events)
    }

    pub async fn get_messages(
        &mut self,
        application: String,
        limit: Option<i32>,
    ) -> SbResult<Vec<Message>> {
        let cmd = GetMessagesCmd {
            header: Some(self.get_header()),
            time_slice: None,
            maybe_application: Some(MaybeApplication::Application(application)),
            limit: limit.unwrap_or(-1),
        };

        self.write_crypto(cmd).await?;

        let m: MessageResponse = self.read_crypto().await?;
        m.try_into()
    }

    pub async fn send_messages(
        &mut self,
        messages: Vec<Message>,
        sign_identity: Option<Uuid>,
    ) -> SbResult<()> {
        let cmd = SendMessageCmd {
            header: Some(self.get_header()),
            messages: messages.into_iter().map(|v| v.into()).collect(),
            sign_identity: sign_identity.map(|v| SignIdentity::Identity(v.as_proto())),
        };

        self.write_crypto(cmd).await?;
        let m: UnitResponse = self.read_crypto().await?;
        m.into_remote_err()?;
        Ok(())
    }

    pub async fn initiate_identity_import(
        &mut self,
        id: Option<Uuid>,
    ) -> SbResult<ImportIdentityState> {
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
    }

    pub async fn get_messages_send_date(
        &mut self,
        application: String,
        limit: Option<i32>,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
    ) -> SbResult<Vec<Message>> {
        let cmd = GetMessagesCmd {
            header: Some(self.get_header()),
            time_slice: Some(TimeSlice::SendDate(TimeRange {
                start: start_date.and_utc().timestamp(),
                end: end_date.and_utc().timestamp(),
            })),
            maybe_application: Some(MaybeApplication::Application(application)),
            limit: limit.unwrap_or(-1),
        };

        self.write_crypto(cmd).await?;

        let m: MessageResponse = self.read_crypto().await?;
        m.try_into()
    }

    pub async fn get_messages_recieve_date(
        &mut self,
        application: String,
        limit: Option<i32>,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
    ) -> SbResult<Vec<Message>> {
        let cmd = GetMessagesCmd {
            header: Some(self.get_header()),
            time_slice: Some(TimeSlice::ReceiveDate(TimeRange {
                start: start_date.and_utc().timestamp(),
                end: end_date.and_utc().timestamp(),
            })),
            maybe_application: Some(MaybeApplication::Application(application)),
            limit: limit.unwrap_or(-1),
        };

        self.write_crypto(cmd).await?;

        let m: MessageResponse = self.read_crypto().await?;
        m.try_into()
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

    pub async fn pair<'a, F, Fut>(
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
            cb(words).await?; // I hate HRTBs
            let v: CryptoMessage = self.read_message().await?;

            let v = CryptoMessageWrapper::new(v);

            let v: Ack = v.decrypt(&tx)?;

            log::info!("got ack {}", v.success);

            if !v.success {
                return Err(Error::PairingFailed);
            }

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
