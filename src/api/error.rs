use std::array::TryFromSliceError;

use crate::proto::{unit_response::UnitresponseMaybeMessage, RespCode, UnitResponse};

pub type SbResult<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Mdns error: {0}")]
    MdnsError(#[from] mdns_sd::Error),
    #[error("Protobuf decode error: {0}")]
    ProtoDecode(#[from] prost::DecodeError),
    #[error("Protobuf encode error: {0}")]
    ProtoEncode(#[from] prost::EncodeError),
    #[error("Unknown enum varient: {0}")]
    ProtoUnknownEnum(#[from] prost::UnknownEnumValue),
    #[error("IO error {0}")]
    IoError(#[from] std::io::Error),
    #[error("Message size error: {0}")]
    MessageSizeError(usize),
    #[error("CRC Mismatch")]
    CrcMismatch,
    #[error("Message type mismatch expected {expected} got {actual}")]
    TypeMismatch { expected: String, actual: String },
    #[error("crypto error: {0}")]
    Crypto(String),
    #[error("Buf length mismatch")]
    BufLengthError(#[from] TryFromSliceError),
    #[error("No addresses remaining")]
    NoAddr,
    #[error("Pairing failed")]
    PairingFailed,
    #[error("Coinscam error: {0}")]
    CoinError(#[from] bip39::Error),
    #[error("Possible mitm detected!")]
    MitmDetected,
    #[error("Corrupt header")]
    CorruptHeader,
    #[error("{0}")]
    RemoteError(String),
    #[error("{0}")]
    JsonError(#[from] serde_json::Error),
    #[error("{0}")]
    Generic(#[from] Box<dyn std::error::Error + Send + Sync>),
}

pub trait IntoRemoteErr {
    fn into_remote_err(self) -> SbResult<()>;
}

impl IntoRemoteErr for UnitResponse {
    fn into_remote_err(self) -> SbResult<()> {
        if self.code() != RespCode::Ok {
            Err(self
                .unitresponse_maybe_message
                .map(|v| {
                    let UnitresponseMaybeMessage::MessageCode(v) = v;
                    Error::RemoteError(v)
                })
                .unwrap_or_else(|| Error::RemoteError("".to_owned())))
        } else {
            Ok(())
        }
    }
}
