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
pub struct SMSG_LOOT_START_ROLL {
    pub creature_guid: Guid,
    pub loot_slot: u32,
    pub item_id: u32,
    pub item_random_suffix: u32,
    pub item_random_property_id: u32,
    pub countdown_time: u32,
}

impl ServerMessageWrite for SMSG_LOOT_START_ROLL {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_LOOT_START_ROLL {
    const OPCODE: u16 = 0x02a1;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // creature_guid: Guid
        let creature_guid = Guid::read(r)?;

        // loot_slot: u32
        let loot_slot = crate::util::read_u32_le(r)?;

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // item_random_suffix: u32
        let item_random_suffix = crate::util::read_u32_le(r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(r)?;

        // countdown_time: u32
        let countdown_time = crate::util::read_u32_le(r)?;

        Ok(Self {
            creature_guid,
            loot_slot,
            item_id,
            item_random_suffix,
            item_random_property_id,
            countdown_time,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // creature_guid: Guid
        self.creature_guid.write(w)?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_random_suffix: u32
        w.write_all(&self.item_random_suffix.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // countdown_time: u32
        w.write_all(&self.countdown_time.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // creature_guid: Guid
        let creature_guid = Guid::tokio_read(r).await?;

        // loot_slot: u32
        let loot_slot = crate::util::tokio_read_u32_le(r).await?;

        // item_id: u32
        let item_id = crate::util::tokio_read_u32_le(r).await?;

        // item_random_suffix: u32
        let item_random_suffix = crate::util::tokio_read_u32_le(r).await?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::tokio_read_u32_le(r).await?;

        // countdown_time: u32
        let countdown_time = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            creature_guid,
            loot_slot,
            item_id,
            item_random_suffix,
            item_random_property_id,
            countdown_time,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // creature_guid: Guid
        self.creature_guid.tokio_write(w).await?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes()).await?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes()).await?;

        // item_random_suffix: u32
        w.write_all(&self.item_random_suffix.to_le_bytes()).await?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes()).await?;

        // countdown_time: u32
        w.write_all(&self.countdown_time.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // creature_guid: Guid
        let creature_guid = Guid::astd_read(r).await?;

        // loot_slot: u32
        let loot_slot = crate::util::astd_read_u32_le(r).await?;

        // item_id: u32
        let item_id = crate::util::astd_read_u32_le(r).await?;

        // item_random_suffix: u32
        let item_random_suffix = crate::util::astd_read_u32_le(r).await?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::astd_read_u32_le(r).await?;

        // countdown_time: u32
        let countdown_time = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            creature_guid,
            loot_slot,
            item_id,
            item_random_suffix,
            item_random_property_id,
            countdown_time,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // creature_guid: Guid
        self.creature_guid.astd_write(w).await?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes()).await?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes()).await?;

        // item_random_suffix: u32
        w.write_all(&self.item_random_suffix.to_le_bytes()).await?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes()).await?;

        // countdown_time: u32
        w.write_all(&self.countdown_time.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_LOOT_START_ROLL {}

impl MaximumPossibleSized for SMSG_LOOT_START_ROLL {
    fn maximum_possible_size() -> usize {
        0
        + 8 // creature_guid: Guid
        + 4 // loot_slot: u32
        + 4 // item_id: u32
        + 4 // item_random_suffix: u32
        + 4 // item_random_property_id: u32
        + 4 // countdown_time: u32
    }
}

