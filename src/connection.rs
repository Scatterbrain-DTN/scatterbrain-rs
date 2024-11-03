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
