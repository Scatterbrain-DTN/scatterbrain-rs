pub use crate::proto::SbEvent;
pub use crate::response::{Identity, Message};
use chrono::NaiveDateTime;
pub use core::future::Future;
pub use serde::{Deserialize, Serialize};
pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
pub use std::pin::Pin;
use uuid::Uuid;
#[derive(Serialize, Deserialize)]
pub struct B64SessionState {
    pub secretkey: String,
    pub pubkey: String,
    pub remotekey: Option<String>,
}

pub use crate::proto::{
    Ack, CryptoMessage, GetEvents, GetIdentityCommand, GetMessagesCmd, IdentityResponse,
    ImportIdentityCommand, ImportIdentityResponse, MessageResponse, MessageType, PairingAck,
    PairingInitiate, PairingRequest, SbEvents, SendMessageCmd, TypePrefix, UnitResponse,
};

use super::api::SbResult;

pub type TraitFuture<'a> =
    Pin<Box<dyn Future<Output = SbResult<Vec<Identity>>> + Send + Sync + 'a>>;

pub trait SessionTrait {
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn get_identity<'life0>(&'life0 mut self, id: Option<Uuid>) -> TraitFuture<'life0>;
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn get_events<'life0>(
        &'life0 mut self,
        block: bool,
        count: Option<u32>,
    ) -> Pin<Box<dyn Future<Output = SbResult<Vec<SbEvent>>> + Send + Sync + 'life0>>;
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn get_messages<'life0>(
        &'life0 mut self,
        application: String,
        limit: Option<i32>,
    ) -> Pin<Box<dyn Future<Output = SbResult<Vec<Message>>> + Send + Sync + 'life0>>;
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn send_messages<'life0>(
        &'life0 mut self,
        messages: Vec<Message>,
        sign_identity: Option<Uuid>,
    ) -> Pin<Box<dyn Future<Output = SbResult<()>> + Send + Sync + 'life0>>;
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn initiate_identity_import<'life0>(
        &'life0 mut self,
        id: Option<Uuid>,
    ) -> Pin<Box<dyn Future<Output = SbResult<ImportIdentityState>> + Send + Sync + 'life0>>;
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn get_messages_send_date<'life0>(
        &'life0 mut self,
        application: String,
        limit: Option<i32>,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
    ) -> Pin<Box<dyn Future<Output = SbResult<Vec<Message>>> + Send + Sync + 'life0>>;
    #[must_use]
    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn get_messages_recieve_date<'life0>(
        &'life0 mut self,
        application: String,
        limit: Option<i32>,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime,
    ) -> Pin<Box<dyn Future<Output = SbResult<Vec<Message>>> + Send + Sync + 'life0>>;
}

pub trait GetType {
    fn get_type() -> MessageType;
    fn get_type_message(&self) -> TypePrefix {
        TypePrefix {
            r#type: Self::get_type().into(),
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

#[derive(Debug)]
pub enum ImportIdentityState {
    Initiated(Uuid),
    Complete(Uuid),
}
