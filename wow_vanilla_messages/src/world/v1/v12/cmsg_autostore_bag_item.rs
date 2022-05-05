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
pub struct CMSG_AUTOSTORE_BAG_ITEM {
    pub source_bag: u8,
    pub source_slot: u8,
    pub destination_bag: u8,
}

impl ClientMessageWrite for CMSG_AUTOSTORE_BAG_ITEM {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_AUTOSTORE_BAG_ITEM {
    const OPCODE: u16 = 0x010b;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // source_bag: u8
        let source_bag = crate::util::read_u8_le(r)?;

        // source_slot: u8
        let source_slot = crate::util::read_u8_le(r)?;

        // destination_bag: u8
        let destination_bag = crate::util::read_u8_le(r)?;

        Ok(Self {
            source_bag,
            source_slot,
            destination_bag,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // source_bag: u8
        w.write_all(&self.source_bag.to_le_bytes())?;

        // source_slot: u8
        w.write_all(&self.source_slot.to_le_bytes())?;

        // destination_bag: u8
        w.write_all(&self.destination_bag.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // source_bag: u8
        let source_bag = crate::util::tokio_read_u8_le(r).await?;

        // source_slot: u8
        let source_slot = crate::util::tokio_read_u8_le(r).await?;

        // destination_bag: u8
        let destination_bag = crate::util::tokio_read_u8_le(r).await?;

        Ok(Self {
            source_bag,
            source_slot,
            destination_bag,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // source_bag: u8
        w.write_all(&self.source_bag.to_le_bytes()).await?;

        // source_slot: u8
        w.write_all(&self.source_slot.to_le_bytes()).await?;

        // destination_bag: u8
        w.write_all(&self.destination_bag.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // source_bag: u8
        let source_bag = crate::util::astd_read_u8_le(r).await?;

        // source_slot: u8
        let source_slot = crate::util::astd_read_u8_le(r).await?;

        // destination_bag: u8
        let destination_bag = crate::util::astd_read_u8_le(r).await?;

        Ok(Self {
            source_bag,
            source_slot,
            destination_bag,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // source_bag: u8
        w.write_all(&self.source_bag.to_le_bytes()).await?;

        // source_slot: u8
        w.write_all(&self.source_slot.to_le_bytes()).await?;

        // destination_bag: u8
        w.write_all(&self.destination_bag.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for CMSG_AUTOSTORE_BAG_ITEM {}

impl MaximumPossibleSized for CMSG_AUTOSTORE_BAG_ITEM {
    fn maximum_possible_size() -> usize {
        1 // source_bag: u8
        + 1 // source_slot: u8
        + 1 // destination_bag: u8
    }
}

