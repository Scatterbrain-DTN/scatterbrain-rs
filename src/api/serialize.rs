use std::io::{ErrorKind, Read, Write};

use crc::Crc;
#[cfg(feature = "flutter")]
use flutter_rust_bridge::DartFnFuture;
use prost::{bytes::BufMut, Message};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::constants::{MESSAGE_SIZE_CAP, TYPE_SIZE_CAP};
use crate::error::{Error, IntoRemoteErr, SbResult};
use crate::proto::{self, MessageType, TypePrefix, UnitResponse};
use crate::types::GetType;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

#[cfg(feature = "flutter")]
pub use super::api::SbSession;

// java uses CRC32 from GZIP RFC1952
const JAVA_ALG: crc::Algorithm<u32> = crc::Algorithm {
    width: 32,
    poly: 0x04C11DB7,
    init: 0xFFFFFFFF,
    refin: true,
    refout: true,
    xorout: 0xFFFFFFFF,
    check: 0xaee7,
    residue: 0x0000,
};

pub trait ToUuid {
    fn as_uuid(&self) -> uuid::Uuid;
    fn as_proto(&self) -> proto::ProtoUuid;
}

impl ToUuid for proto::ProtoUuid {
    fn as_uuid(&self) -> uuid::Uuid {
        uuid::Uuid::from_u64_pair(self.upper, self.lower)
    }

    fn as_proto(&self) -> proto::ProtoUuid {
        *self
    }
}

impl ToUuid for uuid::Uuid {
    fn as_uuid(&self) -> uuid::Uuid {
        *self
    }

    fn as_proto(&self) -> proto::ProtoUuid {
        let (upper, lower) = self.as_u64_pair();
        proto::ProtoUuid { upper, lower }
    }
}

pub struct ProtoStream<A> {
    stream: A,
    pub is_disconnected: bool,
    #[cfg(feature = "flutter")]
    pub(crate) on_connect:
        Option<Box<dyn Fn(Option<SbSession>) -> DartFnFuture<()> + Send + Sync + 'static>>,
}

impl<A> Clone for ProtoStream<A>
where
    A: Clone,
{
    fn clone(&self) -> Self {
        ProtoStream {
            stream: self.stream.clone(),
            is_disconnected: self.is_disconnected,
            #[cfg(feature = "flutter")]
            on_connect: None,
        }
    }
}

#[derive(Debug, Default)]
pub struct TypedMessage<M>
where
    M: Message + GetType + Default + Send,
{
    pub message: M,
    pub message_type: MessageType,
}

impl<M> TypedMessage<M>
where
    M: Message + GetType + Default + Send,
{
    pub fn new_typed(message_type: MessageType) -> Self {
        Self {
            message: M::default(),
            message_type,
        }
    }

    pub fn new(message: M) -> Self {
        let message_type = M::get_type();
        Self {
            message,
            message_type,
        }
    }
}

impl<M> Message for TypedMessage<M>
where
    M: Message + GetType + Default + Send,
{
    fn clear(&mut self) {
        self.message.clear()
    }

    fn encode_raw(&self, buf: &mut impl BufMut)
    where
        Self: Sized,
    {
        self.message.encode_raw(buf)
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: ::prost::encoding::WireType,
        buf: &mut impl prost::bytes::Buf,
        ctx: ::prost::encoding::DecodeContext,
    ) -> std::result::Result<(), prost::DecodeError>
    where
        Self: Sized,
    {
        self.message.merge_field(tag, wire_type, buf, ctx)
    }

    fn encode(&self, buf: &mut impl BufMut) -> std::result::Result<(), prost::EncodeError>
    where
        Self: Sized,
    {
        self.message.encode(buf)
    }

    fn encoded_len(&self) -> usize {
        self.message.encoded_len()
    }

    fn encode_to_vec(&self) -> Vec<u8>
    where
        Self: Sized,
    {
        self.message.encode_to_vec()
    }

    fn merge(&mut self, buf: impl prost::bytes::Buf) -> std::result::Result<(), prost::DecodeError>
    where
        Self: Sized,
    {
        self.message.merge(buf)
    }

    fn decode(buf: impl prost::bytes::Buf) -> std::result::Result<Self, prost::DecodeError>
    where
        Self: Default,
    {
        Ok(Self::new(M::decode(buf)?))
    }
}

impl<A> ProtoStream<A>
where
    A: Unpin + Send,
{
    pub fn new(sock: A) -> Self {
        Self {
            stream: sock,
            is_disconnected: false,
            on_connect: None,
        }
    }

    pub fn write_message_sync<M>(&mut self, message: &M) -> SbResult<()>
    where
        M: Message + GetType + Default + Send,
        A: Write,
    {
        let crc = Crc::<u32>::new(&JAVA_ALG);
        let mut digest = crc.digest();
        let message = message.encode_to_vec();
        let size = message.len() as i32;

        let tp = TypePrefix {
            r#type: M::get_type().into(),
        };

        let tp = tp.encode_to_vec();

        let typesize = tp.len() as i32;
        digest.update(&typesize.to_be_bytes());
        digest.update(&size.to_be_bytes());
        digest.update(&tp);
        digest.update(&message);
        self.stream.write_i32::<BigEndian>(typesize)?;
        self.stream.write_i32::<BigEndian>(size)?;

        self.stream.write(&tp)?;
        self.stream.write(&message)?;

        self.stream.write_u32::<BigEndian>(digest.finalize())?;

        Ok(())
    }

    pub async fn write_message<M>(&mut self, message: &M) -> SbResult<()>
    where
        M: Message + GetType + Default + Send,
        A: AsyncWriteExt,
    {
        let crc = Crc::<u32>::new(&JAVA_ALG);
        let mut digest = crc.digest();
        let message = message.encode_to_vec();
        let size = message.len() as i32;

        let tp = TypePrefix {
            r#type: M::get_type().into(),
        };

        let tp = tp.encode_to_vec();

        let typesize = tp.len() as i32;
        digest.update(&typesize.to_be_bytes());
        digest.update(&size.to_be_bytes());
        digest.update(&tp);
        digest.update(&message);
        self.stream.write_i32(typesize).await?;
        self.stream.write_i32(size).await?;

        self.stream.write(&tp).await?;
        self.stream.write(&message).await?;

        self.stream.write_u32(digest.finalize()).await?;

        Ok(())
    }

    pub fn read_message_sync<M>(&mut self) -> SbResult<M>
    where
        M: Message + GetType + Default + Send,
        A: Read,
    {
        let crc = Crc::<u32>::new(&JAVA_ALG);
        let mut digest = crc.digest();

        let typesize: i32 = self.stream.read_i32::<BigEndian>()?;
        let size = self.stream.read_i32::<BigEndian>()?;

        log::debug!("receivied message sizes {} {}", typesize, size);
        digest.update(&typesize.to_be_bytes());
        digest.update(&size.to_be_bytes());

        if size > MESSAGE_SIZE_CAP as i32 {
            return Err(Error::MessageSizeError(size as usize));
        }

        if typesize > TYPE_SIZE_CAP as i32 {
            return Err(Error::MessageSizeError(typesize as usize));
        }

        let mut mb = vec![0; typesize as usize];
        self.stream.read(mb.as_mut_slice())?;
        digest.update(mb.as_slice());
        let tp = TypePrefix::decode(mb.as_slice())?;

        log::debug!(
            "read type prefix: expected={} got={}",
            M::get_type().as_str_name(),
            tp.r#type().as_str_name()
        );

        if M::get_type() != tp.r#type() {
            return Err(Error::TypeMismatch {
                expected: M::get_type().as_str_name().to_owned(),
                actual: tp.r#type().as_str_name().to_owned(),
            });
        }

        let mut mb = vec![0; size as usize];
        self.stream.read(mb.as_mut_slice())?;
        digest.update(mb.as_slice());
        let m = M::decode(mb.as_slice())?;
        let crc = self.stream.read_u32::<BigEndian>()?;
        let mycrc = digest.finalize();
        log::debug!("received CRC thiers={} ours={}", crc, mycrc);
        if crc != mycrc {
            return Err(Error::CrcMismatch);
        }
        Ok(m)
    }

    pub async fn read_message<M>(&mut self) -> SbResult<M>
    where
        M: Message + GetType + Default + Send,
        A: AsyncReadExt,
    {
        match self.read_message_impl().await {
            Ok(m) => Ok(m),
            Err(err) => match err {
                Error::IoError(err) => {
                    match err.kind() {
                        ErrorKind::ConnectionAborted | ErrorKind::UnexpectedEof => {
                            self.is_disconnected = true;
                            if let Some(on_disconnect) = self.on_connect.as_ref() {
                                on_disconnect(None).await;
                            }
                        }
                        _ => (),
                    }
                    Err(Error::IoError(err))
                }
                e => Err(e),
            },
        }
    }

    async fn read_message_impl<M>(&mut self) -> SbResult<M>
    where
        M: Message + GetType + Default + Send,
        A: AsyncReadExt,
    {
        let crc = Crc::<u32>::new(&JAVA_ALG);
        let mut digest = crc.digest();

        let typesize = self.stream.read_i32().await?;
        let size = self.stream.read_i32().await?;

        log::debug!("receivied message sizes {} {}", typesize, size);
        digest.update(&typesize.to_be_bytes());
        digest.update(&size.to_be_bytes());

        if size > MESSAGE_SIZE_CAP as i32 {
            return Err(Error::MessageSizeError(size as usize));
        }

        if typesize > TYPE_SIZE_CAP as i32 {
            return Err(Error::MessageSizeError(typesize as usize));
        }

        let mut mb = vec![0; typesize as usize];
        self.stream.read(mb.as_mut_slice()).await?;
        digest.update(mb.as_slice());
        let tp = TypePrefix::decode(mb.as_slice())?;

        log::debug!(
            "read type prefix: expected={} got={}",
            M::get_type().as_str_name(),
            tp.r#type().as_str_name()
        );

        if M::get_type() != tp.r#type() {
            if tp.r#type() == MessageType::UnitResponse {
                let mut mb = vec![0; size as usize];
                self.stream.read(mb.as_mut_slice()).await?;
                digest.update(mb.as_slice());
                let m = UnitResponse::decode(mb.as_slice())?;
                let crc = self.stream.read_u32().await?;
                let mycrc = digest.finalize();
                log::debug!("received CRC thiers={} ours={}", crc, mycrc);
                if crc != mycrc {
                    return Err(Error::CrcMismatch);
                }
                m.into_remote_err()?;
            }
            return Err(Error::TypeMismatch {
                expected: M::get_type().as_str_name().to_owned(),
                actual: tp.r#type().as_str_name().to_owned(),
            });
        }

        let mut mb = vec![0; size as usize];
        self.stream.read(mb.as_mut_slice()).await?;
        digest.update(mb.as_slice());
        let m = M::decode(mb.as_slice())?;
        let crc = self.stream.read_u32().await?;
        let mycrc = digest.finalize();
        log::debug!("received CRC thiers={} ours={}", crc, mycrc);
        if crc != mycrc {
            return Err(Error::CrcMismatch);
        }
        Ok(m)
    }
}

#[cfg(test)]
mod test {

    use proto::ack::AckMaybeMessage;

    use crate::proto::Ack;

    use super::*;
    #[tokio::test]
    async fn test_kotlin_ack() {
        let _ = env_logger::try_init();
        let st = tokio::fs::File::open("./src/test/ack-stream")
            .await
            .expect("failed to open test file");
        let mut reader = ProtoStream::new(st);
        let mesage: Ack = reader.read_message().await.expect("failed to read message");
        assert!(mesage.success);
    }

    #[tokio::test]
    async fn test_readwrite() {
        let ack = Ack {
            success: true,
            status: 1,
            ack_maybe_message: Some(AckMaybeMessage::Text("tests".to_owned())),
        };

        let (client, server) = tokio::io::duplex(64);
        let mut client = ProtoStream::new(client);
        let mut server = ProtoStream::new(server);
        client.write_message(&ack).await.expect("failed to write");
        let newack: Ack = server.read_message().await.expect("failed to read");

        assert_eq!(ack, newack);
    }

    #[tokio::test]
    async fn test_readwrite_multiple() {
        let ack = Ack {
            success: true,
            status: 1,
            ack_maybe_message: Some(AckMaybeMessage::Text("tests".to_owned())),
        };

        let (client, server) = tokio::io::duplex(64);
        let mut client = ProtoStream::new(client);
        let mut server = ProtoStream::new(server);
        for _ in 0..20 {
            client.write_message(&ack).await.expect("failed to write");
            let newack: Ack = server.read_message().await.expect("failed to read");

            assert_eq!(ack, newack);
        }
    }

    #[tokio::test]
    async fn test_readwrite_sync() {
        let ack = Ack {
            success: true,
            status: 1,
            ack_maybe_message: Some(AckMaybeMessage::Text("tests".to_owned())),
        };

        let (mut c, server) = tokio::io::duplex(64);
        let mut v = Vec::new();
        let mut client = ProtoStream::new(&mut v);
        let mut server = ProtoStream::new(server);
        client.write_message_sync(&ack).expect("failed to write");
        c.write(v.as_slice()).await.unwrap();
        let newack: Ack = server.read_message().await.expect("failed to read");

        assert_eq!(ack, newack);
    }
}
