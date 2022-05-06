use std::convert::{TryFrom, TryInto};
use crate::Guid;
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
pub struct SMSG_DUEL_REQUESTED {
    pub initiator_guid: Guid,
    pub target_guid: Guid,
}

impl ServerMessageWrite for SMSG_DUEL_REQUESTED {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_DUEL_REQUESTED {
    const OPCODE: u16 = 0x0167;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // initiator_guid: Guid
        let initiator_guid = Guid::read(r)?;

        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        Ok(Self {
            initiator_guid,
            target_guid,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // initiator_guid: Guid
        self.initiator_guid.write(w)?;

        // target_guid: Guid
        self.target_guid.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // initiator_guid: Guid
        let initiator_guid = Guid::tokio_read(r).await?;

        // target_guid: Guid
        let target_guid = Guid::tokio_read(r).await?;

        Ok(Self {
            initiator_guid,
            target_guid,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // initiator_guid: Guid
        self.initiator_guid.tokio_write(w).await?;

        // target_guid: Guid
        self.target_guid.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // initiator_guid: Guid
        let initiator_guid = Guid::astd_read(r).await?;

        // target_guid: Guid
        let target_guid = Guid::astd_read(r).await?;

        Ok(Self {
            initiator_guid,
            target_guid,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // initiator_guid: Guid
        self.initiator_guid.astd_write(w).await?;

        // target_guid: Guid
        self.target_guid.astd_write(w).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_DUEL_REQUESTED {}

impl MaximumPossibleSized for SMSG_DUEL_REQUESTED {
    fn maximum_possible_size() -> usize {
        0
        + 8 // initiator_guid: Guid
        + 8 // target_guid: Guid
    }
}

