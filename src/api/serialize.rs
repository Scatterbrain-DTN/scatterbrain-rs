use std::io::{Read, Write};

use crc::Crc;
use prost::{bytes::BufMut, Message};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::constants::{MESSAGE_SIZE_CAP, TYPE_SIZE_CAP};
use crate::error::{Error, IntoRemoteErr, SbResult};
use crate::proto::{self, MessageType, TypePrefix, UnitResponse};
use crate::types::GetType;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

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

pub struct ProtoStream<A>(A);

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
        Self(sock)
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
        self.0.write_i32::<BigEndian>(typesize)?;
        self.0.write_i32::<BigEndian>(size)?;

        self.0.write(&tp)?;
        self.0.write(&message)?;

        self.0.write_u32::<BigEndian>(digest.finalize())?;

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
        self.0.write_i32(typesize).await?;
        self.0.write_i32(size).await?;

        self.0.write(&tp).await?;
        self.0.write(&message).await?;

        self.0.write_u32(digest.finalize()).await?;

        Ok(())
    }

    pub fn read_message_sync<M>(&mut self) -> SbResult<M>
    where
        M: Message + GetType + Default + Send,
        A: Read,
    {
        let crc = Crc::<u32>::new(&JAVA_ALG);
        let mut digest = crc.digest();

        let typesize: i32 = self.0.read_i32::<BigEndian>()?;
        let size = self.0.read_i32::<BigEndian>()?;

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
        self.0.read(mb.as_mut_slice())?;
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
        self.0.read(mb.as_mut_slice())?;
        digest.update(mb.as_slice());
        let m = M::decode(mb.as_slice())?;
        let crc = self.0.read_u32::<BigEndian>()?;
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
        let crc = Crc::<u32>::new(&JAVA_ALG);
        let mut digest = crc.digest();

        let typesize = self.0.read_i32().await?;
        let size = self.0.read_i32().await?;

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
        self.0.read(mb.as_mut_slice()).await?;
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
                self.0.read(mb.as_mut_slice()).await?;
                digest.update(mb.as_slice());
                let m = UnitResponse::decode(mb.as_slice())?;
                let crc = self.0.read_u32().await?;
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
        self.0.read(mb.as_mut_slice()).await?;
        digest.update(mb.as_slice());
        let m = M::decode(mb.as_slice())?;
        let crc = self.0.read_u32().await?;
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

    use crate::proto::{ack::Message, Ack};

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
            message: Some(Message::Text("tests".to_owned())),
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
            message: Some(Message::Text("tests".to_owned())),
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
            message: Some(Message::Text("tests".to_owned())),
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
