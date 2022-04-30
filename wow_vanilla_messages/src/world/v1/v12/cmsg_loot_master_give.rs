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
pub struct CMSG_LOOT_MASTER_GIVE {
    pub loot_guid: Guid,
    pub slot_id: u8,
    pub target_player_guid: Guid,
}

impl ClientMessageWrite for CMSG_LOOT_MASTER_GIVE {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_LOOT_MASTER_GIVE {
    const OPCODE: u16 = 0x02a3;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // loot_guid: Guid
        let loot_guid = Guid::read(r)?;

        // slot_id: u8
        let slot_id = crate::util::read_u8_le(r)?;

        // target_player_guid: Guid
        let target_player_guid = Guid::read(r)?;

        Ok(Self {
            loot_guid,
            slot_id,
            target_player_guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // loot_guid: Guid
        self.loot_guid.write(w)?;

        // slot_id: u8
        w.write_all(&self.slot_id.to_le_bytes())?;

        // target_player_guid: Guid
        self.target_player_guid.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // loot_guid: Guid
        let loot_guid = Guid::tokio_read(r).await?;

        // slot_id: u8
        let slot_id = crate::util::tokio_read_u8_le(r).await?;

        // target_player_guid: Guid
        let target_player_guid = Guid::tokio_read(r).await?;

        Ok(Self {
            loot_guid,
            slot_id,
            target_player_guid,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // loot_guid: Guid
        self.loot_guid.tokio_write(w).await?;

        // slot_id: u8
        w.write_all(&self.slot_id.to_le_bytes()).await?;

        // target_player_guid: Guid
        self.target_player_guid.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // loot_guid: Guid
        let loot_guid = Guid::astd_read(r).await?;

        // slot_id: u8
        let slot_id = crate::util::astd_read_u8_le(r).await?;

        // target_player_guid: Guid
        let target_player_guid = Guid::astd_read(r).await?;

        Ok(Self {
            loot_guid,
            slot_id,
            target_player_guid,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // loot_guid: Guid
        self.loot_guid.astd_write(w).await?;

        // slot_id: u8
        w.write_all(&self.slot_id.to_le_bytes()).await?;

        // target_player_guid: Guid
        self.target_player_guid.astd_write(w).await?;

        Ok(())
    }
}

impl ConstantSized for CMSG_LOOT_MASTER_GIVE {}

impl MaximumPossibleSized for CMSG_LOOT_MASTER_GIVE {
    fn maximum_possible_size() -> usize {
        8 // loot_guid: Guid
        + 1 // slot_id: u8
        + 8 // target_player_guid: Guid
    }
}

