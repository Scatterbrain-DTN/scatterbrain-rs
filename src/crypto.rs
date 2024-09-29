use prost::{bytes::Buf, Message};
use sodiumoxide::{
    base64,
    crypto::{
        kx::{PublicKey, SecretKey, SessionKey},
        secretbox::{open, seal, Key, Nonce},
    },
    randombytes::randombytes,
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;

use serde::{Deserialize, Serialize};
use sodiumoxide::crypto::kx::{self};

use crate::{
    error::{Error, SbResult},
    proto::{ApiHeader, CryptoMessage},
    serialize::{ProtoStream, ToUuid},
    types::{CryptoConfig, GetType},
};

pub trait EncodeB64<T>
where
    Self: Sized,
{
    fn b64(&self) -> T;
    fn from_b64(val: T) -> SbResult<Self>;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SessionState {
    pub secretkey: SecretKey,
    pub pubkey: PublicKey,
    pub remotekey: Option<PublicKey>,
}

impl EncodeB64<CryptoConfig> for SessionState {
    fn b64(&self) -> CryptoConfig {
        let secretkey = base64::encode(&self.secretkey.0, base64::Variant::UrlSafe);
        let pubkey = base64::encode(&self.pubkey.0, base64::Variant::UrlSafe);
        let remotekey = self
            .remotekey
            .map(|v| base64::encode(&v.0, base64::Variant::UrlSafe));
        CryptoConfig {
            secretkey,
            pubkey,
            remotekey,
        }
    }

    fn from_b64(val: CryptoConfig) -> SbResult<Self> {
        let secretkey = base64::decode(&val.secretkey, base64::Variant::UrlSafe)
            .map_err(|_| Error::Crypto("failed to parse base64".to_owned()))?;
        let pubkey = base64::decode(&val.pubkey, base64::Variant::UrlSafe)
            .map_err(|_| Error::Crypto("failed to parse base64".to_owned()))?;
        let remotekey = val
            .remotekey
            .map(|v| {
                base64::decode(&v, base64::Variant::UrlSafe)
                    .map_err(|_| Error::Crypto("failed to parse base64".to_owned()))
            })
            .transpose()?;
        Ok(Self {
            secretkey: SecretKey(
                secretkey
                    .try_into()
                    .map_err(|_| Error::Crypto("Secret key wrong size".to_owned()))?,
            ),
            pubkey: PublicKey(
                pubkey
                    .try_into()
                    .map_err(|_| Error::Crypto("Public key wrong size".to_owned()))?,
            ),
            remotekey: remotekey
                .map(|k| {
                    Ok::<PublicKey, Error>(PublicKey(
                        k.try_into()
                            .map_err(|_| Error::Crypto("Public key wrong size".to_owned()))?,
                    ))
                })
                .transpose()?,
        })
    }
}

impl Default for SessionState {
    fn default() -> Self {
        let (pubkey, secretkey) = kx::gen_keypair();
        Self {
            pubkey,
            secretkey,
            remotekey: None,
        }
    }
}

pub struct Session<A> {
    pub session: Uuid,
    pub rx: SessionKey,
    pub tx: SessionKey,
    pub state: SessionState,
    pub stream: ProtoStream<A>,
}

impl<A> Session<A>
where
    A: Unpin + Send + AsyncReadExt + AsyncWriteExt,
{
    pub(crate) fn get_header(&self) -> ApiHeader {
        ApiHeader {
            session: Some(self.session.as_proto()),
            stream: None,
        }
    }

    pub async fn write_crypto<M>(&mut self, message: M) -> SbResult<()>
    where
        M: Message + GetType + Default,
    {
        let cm = CryptoMessageWrapper::new_message(&message, &self.rx)?;
        self.stream.write_message(cm.message()).await
    }

    pub async fn read_crypto<M>(&mut self) -> SbResult<M>
    where
        M: Message + GetType + Default,
    {
        let cm: CryptoMessage = self.stream.read_message().await?;
        let w = CryptoMessageWrapper::new(cm);
        w.decrypt(&self.tx)
    }
}

pub fn hash_as_uuid(bytes: &[u8]) -> SbResult<Uuid> {
    let lower: [u8; 8] = bytes[..8].try_into()?;
    let upper: [u8; 8] = bytes[8..16].try_into()?;
    let lower = u64::from_be_bytes(lower);
    let upper = u64::from_be_bytes(upper);

    Ok(Uuid::from_u64_pair(upper, lower))
}

pub struct CryptoMessageWrapper(CryptoMessage);

impl CryptoMessageWrapper {
    pub fn new(cm: CryptoMessage) -> Self {
        Self(cm)
    }

    pub fn message(&self) -> &'_ CryptoMessage {
        &self.0
    }

    pub fn new_message<M>(message: &M, key: &SessionKey) -> SbResult<Self>
    where
        M: Message + GetType + Default + Send,
    {
        let nonce = randombytes(24);
        let n: [u8; 24] = nonce.clone().try_into().unwrap();
        let n = Nonce(n);
        let mut m = Vec::new();
        ProtoStream::new(&mut m).write_message_sync(message)?;
        assert_ne!(m.len(), 0);
        let m = seal(&m, &n, &Key(key.0));
        Ok(Self(CryptoMessage {
            nonce,
            encrypted: m,
        }))
    }

    pub fn decrypt<M>(self, key: &SessionKey) -> SbResult<M>
    where
        M: Message + GetType + Default + Send,
    {
        let nonce: [u8; 24] = self
            .0
            .nonce
            .try_into()
            .map_err(|_| Error::Crypto("Nonce wrong size".to_owned()))?;
        let nonce = Nonce(nonce);
        let bytes = open(&self.0.encrypted, &nonce, &Key(key.0))
            .map_err(|_| Error::Crypto("Decrypt failed".to_owned()))?;
        let mut m = ProtoStream::new(bytes.reader());
        Ok(m.read_message_sync()?)
    }
}

#[cfg(test)]
mod tests {
    use kx::client_session_keys;

    use crate::proto::{ack::Message, Ack};

    use super::{hash_as_uuid, *};

    #[test]
    fn crypto_message() {
        let m = Ack {
            success: true,
            status: 100,
            message: Some(Message::Text("()".to_owned())),
        };

        let (p, sec) = kx::gen_keypair();

        let (key, _) = client_session_keys(&p, &sec, &p).unwrap();

        let cm = CryptoMessageWrapper::new_message(&m, &key).expect("failed to encrypt");
        let nm: Ack = cm.decrypt(&key).expect("failed to decrypt");
        assert!(nm.success);
        assert_eq!(m, nm);
    }

    #[test]
    fn hash_as_uuid_test() {
        let bytes = [
            0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x1,
            0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8,
        ];

        let uuid = hash_as_uuid(&bytes).expect("failed to generate uuid");

        let nuuid = Uuid::parse_str("01020304-0506-0708-0102-030405060708").unwrap();
        assert_eq!(uuid.to_string(), nuuid.to_string())
    }
}
