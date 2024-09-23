use std::{collections::HashMap, fmt};

use serde::{ser::Error, Deserialize, Serialize};
use uuid::Uuid;

pub use crate::{
    error::{Error as CrateError, SbResult},
    proto::{ApiMessage, IdentityResponse, MessageResponse, RespCode},
    serialize::ToUuid,
};

pub trait PrettyPrint {
    fn print_output(&self) -> SbResult<String>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Identity {
    pub fingerprint: Option<Uuid>,
    pub name: String,
    pub public_key: Vec<u8>,
    pub is_owned: bool,
    pub extra: HashMap<String, Vec<u8>>,
    pub sig: Vec<u8>,
}

impl PrettyPrint for Vec<Identity> {
    fn print_output(&self) -> SbResult<String> {
        let s = serde_json::to_string_pretty(self)?;

        Ok(s)
    }
}

impl fmt::Display for Identity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s =
            serde_json::to_string_pretty(self).map_err(|e| fmt::Error::custom(e.to_string()))?;
        f.write_str(&s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub from_fingerprint: Option<Uuid>,
    pub to_fingerprint: Option<Uuid>,
    pub application: String,
    pub extension: String,
    pub mime: String,
    pub send_date: i64,
    pub receive_date: i64,
    pub is_file: bool,
    pub id: Option<Uuid>,
    pub body: Vec<u8>,
    pub file_name: String,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s =
            serde_json::to_string_pretty(self).map_err(|e| fmt::Error::custom(e.to_string()))?;
        f.write_str(&s)
    }
}

impl PrettyPrint for Vec<Message> {
    fn print_output(&self) -> SbResult<String> {
        let s = serde_json::to_string_pretty(self)?;

        Ok(s)
    }
}

impl TryFrom<IdentityResponse> for Vec<Identity> {
    type Error = CrateError;
    fn try_from(value: IdentityResponse) -> std::result::Result<Self, Self::Error> {
        if let RespCode::Ok = value.code.try_into()? {
            Ok(value
                .identity
                .into_iter()
                .map(|v| Identity {
                    fingerprint: v.fingerprint.map(|v| v.as_uuid()),
                    name: v.name,
                    public_key: v.public_key,
                    is_owned: v.is_owned,
                    extra: v.extra,
                    sig: v.sig,
                })
                .collect())
        } else {
            Err(CrateError::RemoteError(value.code.to_string()))
        }
    }
}

impl TryFrom<MessageResponse> for Vec<Message> {
    type Error = CrateError;
    fn try_from(value: MessageResponse) -> std::result::Result<Self, Self::Error> {
        if let RespCode::Ok = value.code.try_into()? {
            Ok(value
                .messsage
                .into_iter()
                .map(|v| Message {
                    from_fingerprint: v.from_fingerprint.map(|v| v.as_uuid()),
                    to_fingerprint: v.to_fingerprint.map(|v| v.as_uuid()),
                    application: v.application,
                    mime: v.mime,
                    extension: v.extension,
                    send_date: v.send_date,
                    receive_date: v.receive_date,
                    is_file: v.is_file,
                    id: v.id.map(|v| v.as_uuid()),
                    body: v.body,
                    file_name: v.file_name,
                })
                .collect())
        } else {
            Err(CrateError::RemoteError(value.code.to_string()))
        }
    }
}

impl From<Message> for ApiMessage {
    fn from(v: Message) -> Self {
        ApiMessage {
            from_fingerprint: v.from_fingerprint.map(|v| v.as_proto()),
            to_fingerprint: v.to_fingerprint.map(|v| v.as_proto()),
            application: v.application,
            mime: v.mime,
            extension: v.extension,
            send_date: v.send_date,
            receive_date: v.receive_date,
            is_file: v.is_file,
            id: v.id.map(|v| v.as_proto()),
            body: v.body,
            file_name: v.file_name,
        }
    }
}
