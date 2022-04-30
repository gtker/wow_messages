use std::convert::{TryFrom, TryInto};
use crate::Guid;
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
pub struct CMSG_AUTOEQUIP_ITEM_SLOT {
    pub guid: Guid,
    pub destination_slot: u8,
}

impl ClientMessageWrite for CMSG_AUTOEQUIP_ITEM_SLOT {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_AUTOEQUIP_ITEM_SLOT {
    const OPCODE: u16 = 0x010f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // destination_slot: u8
        let destination_slot = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            destination_slot,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // destination_slot: u8
        w.write_all(&self.destination_slot.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::tokio_read(r).await?;

        // destination_slot: u8
        let destination_slot = crate::util::tokio_read_u8_le(r).await?;

        Ok(Self {
            guid,
            destination_slot,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.tokio_write(w).await?;

        // destination_slot: u8
        w.write_all(&self.destination_slot.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::astd_read(r).await?;

        // destination_slot: u8
        let destination_slot = crate::util::astd_read_u8_le(r).await?;

        Ok(Self {
            guid,
            destination_slot,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.astd_write(w).await?;

        // destination_slot: u8
        w.write_all(&self.destination_slot.to_le_bytes()).await?;

        Ok(())
    }
}

impl ConstantSized for CMSG_AUTOEQUIP_ITEM_SLOT {}

impl MaximumPossibleSized for CMSG_AUTOEQUIP_ITEM_SLOT {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 1 // destination_slot: u8
    }
}

