pub use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use uuid::Uuid;

pub use crate::proto::{
    Ack, CryptoMessage, GetEvents, GetIdentityCommand, GetMessagesCmd, IdentityResponse,
    ImportIdentityCommand, ImportIdentityResponse, MessageResponse, MessageType, PairingAck,
    PairingInitiate, PairingRequest, SbEvents, SendMessageCmd, TypePrefix, UnitResponse,
};

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
