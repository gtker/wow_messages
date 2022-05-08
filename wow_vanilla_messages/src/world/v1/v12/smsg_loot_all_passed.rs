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
pub struct SMSG_LOOT_ALL_PASSED {
    pub looted_target_guid: Guid,
    pub loot_slot: u32,
    pub item_id: u32,
    pub item_random_property_id: u32,
    pub item_random_suffix_id: u32,
}

impl ServerMessageWrite for SMSG_LOOT_ALL_PASSED {}

impl MessageBody for SMSG_LOOT_ALL_PASSED {
    const OPCODE: u16 = 0x029e;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // looted_target_guid: Guid
        let looted_target_guid = Guid::read(r)?;

        // loot_slot: u32
        let loot_slot = crate::util::read_u32_le(r)?;

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(r)?;

        // item_random_suffix_id: u32
        let item_random_suffix_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            looted_target_guid,
            loot_slot,
            item_id,
            item_random_property_id,
            item_random_suffix_id,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // looted_target_guid: Guid
        self.looted_target_guid.write(w)?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // item_random_suffix_id: u32
        w.write_all(&self.item_random_suffix_id.to_le_bytes())?;

        Ok(())
    }

    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // looted_target_guid: Guid
            let looted_target_guid = Guid::tokio_read(r).await?;

            // loot_slot: u32
            let loot_slot = crate::util::tokio_read_u32_le(r).await?;

            // item_id: u32
            let item_id = crate::util::tokio_read_u32_le(r).await?;

            // item_random_property_id: u32
            let item_random_property_id = crate::util::tokio_read_u32_le(r).await?;

            // item_random_suffix_id: u32
            let item_random_suffix_id = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                looted_target_guid,
                loot_slot,
                item_id,
                item_random_property_id,
                item_random_suffix_id,
            })
        })
    }

    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // looted_target_guid: Guid
            self.looted_target_guid.tokio_write(w).await?;

            // loot_slot: u32
            w.write_all(&self.loot_slot.to_le_bytes()).await?;

            // item_id: u32
            w.write_all(&self.item_id.to_le_bytes()).await?;

            // item_random_property_id: u32
            w.write_all(&self.item_random_property_id.to_le_bytes()).await?;

            // item_random_suffix_id: u32
            w.write_all(&self.item_random_suffix_id.to_le_bytes()).await?;

            Ok(())
        })
    }

    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // looted_target_guid: Guid
            let looted_target_guid = Guid::astd_read(r).await?;

            // loot_slot: u32
            let loot_slot = crate::util::astd_read_u32_le(r).await?;

            // item_id: u32
            let item_id = crate::util::astd_read_u32_le(r).await?;

            // item_random_property_id: u32
            let item_random_property_id = crate::util::astd_read_u32_le(r).await?;

            // item_random_suffix_id: u32
            let item_random_suffix_id = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                looted_target_guid,
                loot_slot,
                item_id,
                item_random_property_id,
                item_random_suffix_id,
            })
        })
    }

    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // looted_target_guid: Guid
            self.looted_target_guid.astd_write(w).await?;

            // loot_slot: u32
            w.write_all(&self.loot_slot.to_le_bytes()).await?;

            // item_id: u32
            w.write_all(&self.item_id.to_le_bytes()).await?;

            // item_random_property_id: u32
            w.write_all(&self.item_random_property_id.to_le_bytes()).await?;

            // item_random_suffix_id: u32
            w.write_all(&self.item_random_suffix_id.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for SMSG_LOOT_ALL_PASSED {}

impl MaximumPossibleSized for SMSG_LOOT_ALL_PASSED {
    fn maximum_possible_size() -> usize {
        0
        + 8 // looted_target_guid: Guid
        + 4 // loot_slot: u32
        + 4 // item_id: u32
        + 4 // item_random_property_id: u32
        + 4 // item_random_suffix_id: u32
    }
}

