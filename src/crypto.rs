use dryoc::{
    constants::CRYPTO_KX_SESSIONKEYBYTES,
    dryocsecretbox::{DryocSecretBox, VecBox},
    kx::{KeyPair, PublicKey},
    rng::randombytes_buf,
    types::{ByteArray, StackByteArray},
};
#[cfg(feature = "flutter")]
use flutter_rust_bridge::frb;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use prost::{bytes::Buf, Message};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use crate::{
    api::proto::{ApiHeader, CryptoMessage},
    error::{Error, SbResult},
    serialize::{ProtoStream, ToUuid},
    types::{CryptoConfig, GetType},
};
#[cfg_attr(feature = "flutter", frb(ignore))]
pub trait EncodeB64<T>
where
    Self: Sized,
{
    fn b64(&self) -> T;
    fn from_b64(val: T) -> SbResult<Self>;
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "flutter", frb(opaque))]
pub struct SessionState {
    pub kp: KeyPair,
    pub remotekey: Option<PublicKey>,
}

#[cfg_attr(feature = "flutter", frb(ignore))]
impl EncodeB64<CryptoConfig> for SessionState {
    fn b64(&self) -> CryptoConfig {
        let secretkey = URL_SAFE.encode(&self.kp.secret_key);
        let pubkey = URL_SAFE.encode(&self.kp.public_key);
        let remotekey = self.remotekey.as_ref().map(|v| URL_SAFE.encode(v));
        CryptoConfig {
            secretkey,
            pubkey,
            remotekey,
        }
    }

    fn from_b64(val: CryptoConfig) -> SbResult<Self> {
        let secretkey = URL_SAFE
            .decode(&val.secretkey)
            .map_err(|_| Error::Crypto("failed to parse base64".to_owned()))?;
        let pubkey = URL_SAFE
            .decode(&val.pubkey)
            .map_err(|_| Error::Crypto("failed to parse base64".to_owned()))?;
        let remotekey = val
            .remotekey
            .map(|v| {
                URL_SAFE
                    .decode(&v)
                    .map_err(|_| Error::Crypto("failed to parse base64".to_owned()))
            })
            .transpose()?;
        Ok(Self {
            kp: KeyPair {
                secret_key: secretkey
                    .as_slice()
                    .try_into()
                    .map_err(|_| Error::Crypto("Secret key wrong size".to_owned()))?,

                public_key: pubkey
                    .as_slice()
                    .try_into()
                    .map_err(|_| Error::Crypto("Public key wrong size".to_owned()))?,
            },
            remotekey: remotekey
                .map(|k| {
                    Ok::<PublicKey, Error>(
                        k.as_slice()
                            .try_into()
                            .map_err(|_| Error::Crypto("Public key wrong size".to_owned()))?,
                    )
                })
                .transpose()?,
        })
    }
}

impl Default for SessionState {
    fn default() -> Self {
        let kp = KeyPair::gen();
        Self {
            kp,
            remotekey: None,
        }
    }
}

pub type KxSession =
    dryoc::kx::Session<StackByteArray<{ dryoc::constants::CRYPTO_KX_SESSIONKEYBYTES }>>;

pub struct Session<A> {
    pub session: Uuid,
    pub session_keys: KxSession,
    pub state: SessionState,
    pub stream: ProtoStream<A>,
}

impl<A> Session<A>
where
    A: Unpin + Send + AsyncReadExt + AsyncWriteExt,
{
    pub fn is_disconnected(&self) -> bool {
        self.stream.is_disconnected
    }

    pub fn get_header(&self) -> ApiHeader {
        ApiHeader {
            session: Some(self.session.as_proto()),
            stream: None,
        }
    }

    pub async fn write_crypto<M>(&mut self, message: M) -> SbResult<()>
    where
        M: Message + GetType + Default,
    {
        let cm = CryptoMessageWrapper::new_message(&message, self.session_keys.rx_as_array())?;
        self.stream.write_message(cm.message()).await
    }

    pub async fn read_crypto<M>(&mut self) -> SbResult<M>
    where
        M: Message + GetType + Default,
    {
        let cm: CryptoMessage = self.stream.read_message().await?;
        let w = CryptoMessageWrapper::new(cm);
        w.decrypt(self.session_keys.tx_as_array())
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

    pub fn new_message<M>(message: &M, key: &[u8; CRYPTO_KX_SESSIONKEYBYTES]) -> SbResult<Self>
    where
        M: Message + GetType + Default + Send,
    {
        let nonce = randombytes_buf(24);
        let n: [u8; 24] = nonce.clone().try_into().unwrap();
        let mut m = Vec::new();
        ProtoStream::new(&mut m).write_message_sync(message)?;
        assert_ne!(m.len(), 0);
        let m = DryocSecretBox::encrypt_to_vecbox(&m, &n, key.as_array());
        let m = m.to_vec();
        Ok(Self(CryptoMessage {
            nonce,
            encrypted: m,
        }))
    }

    pub fn decrypt<M>(self, key: &[u8; CRYPTO_KX_SESSIONKEYBYTES]) -> SbResult<M>
    where
        M: Message + GetType + Default + Send,
    {
        let nonce: [u8; 24] = self
            .0
            .nonce
            .try_into()
            .map_err(|_| Error::Crypto("Nonce wrong size".to_owned()))?;
        let bytes = VecBox::from_bytes(&self.0.encrypted)
            .map_err(|_| Error::Crypto("Decrypt vecbox failed".to_owned()))?;

        let bytes = bytes
            .decrypt_to_vec(&nonce, key.as_array())
            .map_err(|_| Error::Crypto("Decrypt failed".to_owned()))?;
        let mut m = ProtoStream::new(bytes.reader());
        Ok(m.read_message_sync()?)
    }
}

#[cfg(test)]
mod tests {
    use super::{hash_as_uuid, *};
    use crate::api::proto::{ack::AckMaybeMessage, Ack};

    #[test]
    fn crypto_message() {
        let m = Ack {
            success: true,
            status: 100,
            ack_maybe_message: Some(AckMaybeMessage::Text("()".to_owned())),
        };

        let kp = KeyPair::gen();
        let remote = KeyPair::gen();

        let key: KxSession = dryoc::kx::Session::new_client(&kp, &remote.public_key).unwrap();

        let cm =
            CryptoMessageWrapper::new_message(&m, key.tx_as_array()).expect("failed to encrypt");
        let nm: Ack = cm.decrypt(key.tx_as_array()).expect("failed to decrypt");
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
