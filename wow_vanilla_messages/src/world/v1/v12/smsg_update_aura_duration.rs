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
pub struct SMSG_UPDATE_AURA_DURATION {
    pub aura_slot: u8,
    pub aura_duration: u32,
}

impl ServerMessageWrite for SMSG_UPDATE_AURA_DURATION {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_UPDATE_AURA_DURATION {
    const OPCODE: u16 = 0x0137;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // aura_slot: u8
        let aura_slot = crate::util::read_u8_le(r)?;

        // aura_duration: u32
        let aura_duration = crate::util::read_u32_le(r)?;

        Ok(Self {
            aura_slot,
            aura_duration,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // aura_slot: u8
        w.write_all(&self.aura_slot.to_le_bytes())?;

        // aura_duration: u32
        w.write_all(&self.aura_duration.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // aura_slot: u8
        let aura_slot = crate::util::tokio_read_u8_le(r).await?;

        // aura_duration: u32
        let aura_duration = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            aura_slot,
            aura_duration,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // aura_slot: u8
        w.write_all(&self.aura_slot.to_le_bytes()).await?;

        // aura_duration: u32
        w.write_all(&self.aura_duration.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // aura_slot: u8
        let aura_slot = crate::util::astd_read_u8_le(r).await?;

        // aura_duration: u32
        let aura_duration = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            aura_slot,
            aura_duration,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // aura_slot: u8
        w.write_all(&self.aura_slot.to_le_bytes()).await?;

        // aura_duration: u32
        w.write_all(&self.aura_duration.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_UPDATE_AURA_DURATION {}

impl MaximumPossibleSized for SMSG_UPDATE_AURA_DURATION {
    fn maximum_possible_size() -> usize {
        0
        + 1 // aura_slot: u8
        + 4 // aura_duration: u32
    }
}

