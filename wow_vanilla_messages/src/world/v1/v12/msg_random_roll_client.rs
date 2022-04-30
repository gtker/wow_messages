use std::convert::{TryFrom, TryInto};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct MSG_RANDOM_ROLL_Client {
    pub minimum: u32,
    pub maximum: u32,
}

impl ClientMessageWrite for MSG_RANDOM_ROLL_Client {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for MSG_RANDOM_ROLL_Client {
    const OPCODE: u16 = 0x01fb;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // minimum: u32
        let minimum = crate::util::read_u32_le(r)?;

        // maximum: u32
        let maximum = crate::util::read_u32_le(r)?;

        Ok(Self {
            minimum,
            maximum,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // minimum: u32
        w.write_all(&self.minimum.to_le_bytes())?;

        // maximum: u32
        w.write_all(&self.maximum.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // minimum: u32
        let minimum = crate::util::tokio_read_u32_le(r).await?;

        // maximum: u32
        let maximum = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            minimum,
            maximum,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // minimum: u32
        w.write_all(&self.minimum.to_le_bytes()).await?;

        // maximum: u32
        w.write_all(&self.maximum.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // minimum: u32
        let minimum = crate::util::astd_read_u32_le(r).await?;

        // maximum: u32
        let maximum = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            minimum,
            maximum,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // minimum: u32
        w.write_all(&self.minimum.to_le_bytes()).await?;

        // maximum: u32
        w.write_all(&self.maximum.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for MSG_RANDOM_ROLL_Client {}

impl MaximumPossibleSized for MSG_RANDOM_ROLL_Client {
    fn maximum_possible_size() -> usize {
        4 // minimum: u32
        + 4 // maximum: u32
    }
}

