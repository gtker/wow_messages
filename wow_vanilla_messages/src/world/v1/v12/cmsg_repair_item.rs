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
pub struct CMSG_REPAIR_ITEM {
    pub npc_guid: Guid,
    pub item_guid: Guid,
}

impl ClientMessageWrite for CMSG_REPAIR_ITEM {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_REPAIR_ITEM {
    const OPCODE: u16 = 0x02a8;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc_guid: Guid
        let npc_guid = Guid::read(r)?;

        // item_guid: Guid
        let item_guid = Guid::read(r)?;

        Ok(Self {
            npc_guid,
            item_guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // npc_guid: Guid
        self.npc_guid.write(w)?;

        // item_guid: Guid
        self.item_guid.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc_guid: Guid
        let npc_guid = Guid::tokio_read(r).await?;

        // item_guid: Guid
        let item_guid = Guid::tokio_read(r).await?;

        Ok(Self {
            npc_guid,
            item_guid,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // npc_guid: Guid
        self.npc_guid.tokio_write(w).await?;

        // item_guid: Guid
        self.item_guid.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc_guid: Guid
        let npc_guid = Guid::astd_read(r).await?;

        // item_guid: Guid
        let item_guid = Guid::astd_read(r).await?;

        Ok(Self {
            npc_guid,
            item_guid,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // npc_guid: Guid
        self.npc_guid.astd_write(w).await?;

        // item_guid: Guid
        self.item_guid.astd_write(w).await?;

        Ok(())
    }
}

impl ConstantSized for CMSG_REPAIR_ITEM {}

impl MaximumPossibleSized for CMSG_REPAIR_ITEM {
    fn maximum_possible_size() -> usize {
        8 // npc_guid: Guid
        + 8 // item_guid: Guid
    }
}

