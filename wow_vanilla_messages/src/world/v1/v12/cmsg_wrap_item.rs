use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_WRAP_ITEM {
    pub gift_bag_index: u8,
    pub gift_slot: u8,
    pub item_bag_index: u8,
    pub item_slot: u8,
}

impl CMSG_WRAP_ITEM {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 4], std::io::Error> {
        let mut array_w = [0u8; 4];
        let mut w = array_w.as_mut_slice();
        // gift_bag_index: u8
        w.write_all(&self.gift_bag_index.to_le_bytes())?;

        // gift_slot: u8
        w.write_all(&self.gift_slot.to_le_bytes())?;

        // item_bag_index: u8
        w.write_all(&self.item_bag_index.to_le_bytes())?;

        // item_slot: u8
        w.write_all(&self.item_slot.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ClientMessage for CMSG_WRAP_ITEM {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // gift_bag_index: u8
        w.write_all(&self.gift_bag_index.to_le_bytes())?;

        // gift_slot: u8
        w.write_all(&self.gift_slot.to_le_bytes())?;

        // item_bag_index: u8
        w.write_all(&self.item_bag_index.to_le_bytes())?;

        // item_slot: u8
        w.write_all(&self.item_slot.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01d3;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // gift_bag_index: u8
        let gift_bag_index = crate::util::read_u8_le(r)?;

        // gift_slot: u8
        let gift_slot = crate::util::read_u8_le(r)?;

        // item_bag_index: u8
        let item_bag_index = crate::util::read_u8_le(r)?;

        // item_slot: u8
        let item_slot = crate::util::read_u8_le(r)?;

        Ok(Self {
            gift_bag_index,
            gift_slot,
            item_bag_index,
            item_slot,
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
            // gift_bag_index: u8
            let gift_bag_index = crate::util::tokio_read_u8_le(r).await?;

            // gift_slot: u8
            let gift_slot = crate::util::tokio_read_u8_le(r).await?;

            // item_bag_index: u8
            let item_bag_index = crate::util::tokio_read_u8_le(r).await?;

            // item_slot: u8
            let item_slot = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                gift_bag_index,
                gift_slot,
                item_bag_index,
                item_slot,
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
            // gift_bag_index: u8
            let gift_bag_index = crate::util::astd_read_u8_le(r).await?;

            // gift_slot: u8
            let gift_slot = crate::util::astd_read_u8_le(r).await?;

            // item_bag_index: u8
            let item_bag_index = crate::util::astd_read_u8_le(r).await?;

            // item_slot: u8
            let item_slot = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                gift_bag_index,
                gift_slot,
                item_bag_index,
                item_slot,
            })
        })
    }

}

