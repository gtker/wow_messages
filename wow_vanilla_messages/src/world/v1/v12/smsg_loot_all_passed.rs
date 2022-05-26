use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_LOOT_ALL_PASSED {
    pub looted_target_guid: Guid,
    pub loot_slot: u32,
    pub item_id: u32,
    pub item_random_property_id: u32,
    pub item_random_suffix_id: u32,
}

impl SMSG_LOOT_ALL_PASSED {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 24], std::io::Error> {
        let mut array_w = [0u8; 24];
        let mut w = array_w.as_mut_slice();
        // looted_target_guid: Guid
        w.write_all(&self.looted_target_guid.guid().to_le_bytes())?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // item_random_suffix_id: u32
        w.write_all(&self.item_random_suffix_id.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_LOOT_ALL_PASSED {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(24);
        // looted_target_guid: Guid
        w.write_all(&self.looted_target_guid.guid().to_le_bytes())?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // item_random_suffix_id: u32
        w.write_all(&self.item_random_suffix_id.to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x029e;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        24
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

    #[cfg(feature = "tokio")]
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

    #[cfg(feature = "async-std")]
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

}

