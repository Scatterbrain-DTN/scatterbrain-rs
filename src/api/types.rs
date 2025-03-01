#[cfg(feature = "flutter")]
pub use crate::api::api::SbSession;
use crate::api::proto::PairingSynAck;
pub use crate::response::{Identity, Message};
pub use core::future::Future;
#[cfg(feature = "flutter")]
use flutter_rust_bridge::frb;
#[cfg(feature = "flutter")]
pub use flutter_rust_bridge::DartFnFuture;
pub use serde::{Deserialize, Serialize};
pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub use std::pin::Pin;

use uuid::Uuid;
#[derive(Serialize, Deserialize)]
pub struct CryptoConfig {
    pub secretkey: String,
    pub pubkey: String,
    pub remotekey: Option<String>,
}

impl CryptoConfig {
    pub fn generate() -> CryptoConfig {
        SessionState::default().b64()
    }
}

pub use crate::api::proto::{
    Ack, CryptoMessage, GetEvents, GetIdentityCommand, GetMessagesCmd, IdentityResponse,
    ImportIdentityCommand, ImportIdentityResponse, MessageResponse, MessageType, PairingAck,
    PairingInitiate, PairingRequest, SbEvents, SendMessageCmd, TypePrefix, UnitResponse,
};
#[cfg_attr(feature = "flutter", flutter_rust_bridge::frb(non_opaque))]
pub type DartFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
#[cfg_attr(feature = "flutter", flutter_rust_bridge::frb(non_opaque))]
pub type DartSyncFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>>;

use crate::crypto::{EncodeB64, SessionState};

pub trait GetType {
    fn get_type() -> MessageType;
    fn get_type_message(&self) -> TypePrefix {
        TypePrefix {
            message_type: Self::get_type().into(),
        }
    }
}

impl GetType for GetMessagesCmd {
    fn get_type() -> MessageType {
        MessageType::GetMessage
    }
}

impl GetType for GetIdentityCommand {
    fn get_type() -> MessageType {
        MessageType::GetIdentity
    }
}

impl GetType for SendMessageCmd {
    fn get_type() -> MessageType {
        MessageType::SendMessage
    }
}

impl GetType for MessageType {
    fn get_type() -> MessageType {
        MessageType::Message
    }
}

impl GetType for UnitResponse {
    fn get_type() -> MessageType {
        MessageType::UnitResponse
    }
}

impl GetType for CryptoMessage {
    fn get_type() -> MessageType {
        MessageType::CryptoMessage
    }
}

impl GetType for PairingRequest {
    fn get_type() -> MessageType {
        MessageType::PairingRequest
    }
}

impl GetType for PairingInitiate {
    fn get_type() -> MessageType {
        MessageType::PairingInitiate
    }
}

impl GetType for PairingAck {
    fn get_type() -> MessageType {
        MessageType::PairingAck
    }
}

impl GetType for Ack {
    fn get_type() -> MessageType {
        MessageType::Ack
    }
}

impl GetType for IdentityResponse {
    fn get_type() -> MessageType {
        MessageType::IdentityResponse
    }
}

impl GetType for MessageResponse {
    fn get_type() -> MessageType {
        MessageType::MessageResponse
    }
}

impl GetType for ImportIdentityCommand {
    fn get_type() -> MessageType {
        MessageType::ImportIdentity
    }
}

impl GetType for ImportIdentityResponse {
    fn get_type() -> MessageType {
        MessageType::ImportIdentityResponse
    }
}

impl GetType for GetEvents {
    fn get_type() -> MessageType {
        MessageType::GetEvents
    }
}

impl GetType for SbEvents {
    fn get_type() -> MessageType {
        MessageType::DesktopEvents
    }
}

impl GetType for PairingSynAck {
    fn get_type() -> MessageType {
        MessageType::PairingSynack
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "flutter", frb(non_opaque))]
pub enum ImportIdentityState {
    Initiated(Uuid),
    Complete(Uuid),
}
