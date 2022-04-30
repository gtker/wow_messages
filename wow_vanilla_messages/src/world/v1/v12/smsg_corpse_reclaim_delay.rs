use std::convert::{TryFrom, TryInto};
use crate::{ServerMessageWrite, MessageBody};
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
pub struct SMSG_CORPSE_RECLAIM_DELAY {
    pub delay_in_seconds: u32,
}

impl ServerMessageWrite for SMSG_CORPSE_RECLAIM_DELAY {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_CORPSE_RECLAIM_DELAY {
    const OPCODE: u16 = 0x0269;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // delay_in_seconds: u32
        let delay_in_seconds = crate::util::read_u32_le(r)?;

        Ok(Self {
            delay_in_seconds,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // delay_in_seconds: u32
        w.write_all(&self.delay_in_seconds.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // delay_in_seconds: u32
        let delay_in_seconds = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            delay_in_seconds,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // delay_in_seconds: u32
        w.write_all(&self.delay_in_seconds.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // delay_in_seconds: u32
        let delay_in_seconds = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            delay_in_seconds,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // delay_in_seconds: u32
        w.write_all(&self.delay_in_seconds.to_le_bytes()).await?;

        Ok(())
    }
}

impl ConstantSized for SMSG_CORPSE_RECLAIM_DELAY {}

impl MaximumPossibleSized for SMSG_CORPSE_RECLAIM_DELAY {
    fn maximum_possible_size() -> usize {
        4 // delay_in_seconds: u32
    }
}

