use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_AUCTION_LIST_ITEMS {
    pub auctioneer_guid: Guid,
    pub list_start_item: u32,
    pub searched_name: String,
    pub minimum_level: u8,
    pub maximum_level: u8,
    pub auction_slot_id: u32,
    pub auction_main_category: u32,
    pub auction_sub_category: u32,
    pub auction_quality: u32,
    pub usable: u8,
}

impl CMSG_AUCTION_LIST_ITEMS {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // auctioneer_guid: Guid
        w.write_all(&self.auctioneer_guid.guid().to_le_bytes())?;

        // list_start_item: u32
        w.write_all(&self.list_start_item.to_le_bytes())?;

        // searched_name: CString
        w.write_all(self.searched_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // minimum_level: u8
        w.write_all(&self.minimum_level.to_le_bytes())?;

        // maximum_level: u8
        w.write_all(&self.maximum_level.to_le_bytes())?;

        // auction_slot_id: u32
        w.write_all(&self.auction_slot_id.to_le_bytes())?;

        // auction_main_category: u32
        w.write_all(&self.auction_main_category.to_le_bytes())?;

        // auction_sub_category: u32
        w.write_all(&self.auction_sub_category.to_le_bytes())?;

        // auction_quality: u32
        w.write_all(&self.auction_quality.to_le_bytes())?;

        // usable: u8
        w.write_all(&self.usable.to_le_bytes())?;

        Ok(w)
    }
}

impl ClientMessage for CMSG_AUCTION_LIST_ITEMS {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // auctioneer_guid: Guid
        w.write_all(&self.auctioneer_guid.guid().to_le_bytes())?;

        // list_start_item: u32
        w.write_all(&self.list_start_item.to_le_bytes())?;

        // searched_name: CString
        w.write_all(self.searched_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // minimum_level: u8
        w.write_all(&self.minimum_level.to_le_bytes())?;

        // maximum_level: u8
        w.write_all(&self.maximum_level.to_le_bytes())?;

        // auction_slot_id: u32
        w.write_all(&self.auction_slot_id.to_le_bytes())?;

        // auction_main_category: u32
        w.write_all(&self.auction_main_category.to_le_bytes())?;

        // auction_sub_category: u32
        w.write_all(&self.auction_sub_category.to_le_bytes())?;

        // auction_quality: u32
        w.write_all(&self.auction_quality.to_le_bytes())?;

        // usable: u8
        w.write_all(&self.usable.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0258;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // auctioneer_guid: Guid
        let auctioneer_guid = Guid::read(r)?;

        // list_start_item: u32
        let list_start_item = crate::util::read_u32_le(r)?;

        // searched_name: CString
        let searched_name = crate::util::read_c_string_to_vec(r)?;
        let searched_name = String::from_utf8(searched_name)?;

        // minimum_level: u8
        let minimum_level = crate::util::read_u8_le(r)?;

        // maximum_level: u8
        let maximum_level = crate::util::read_u8_le(r)?;

        // auction_slot_id: u32
        let auction_slot_id = crate::util::read_u32_le(r)?;

        // auction_main_category: u32
        let auction_main_category = crate::util::read_u32_le(r)?;

        // auction_sub_category: u32
        let auction_sub_category = crate::util::read_u32_le(r)?;

        // auction_quality: u32
        let auction_quality = crate::util::read_u32_le(r)?;

        // usable: u8
        let usable = crate::util::read_u8_le(r)?;

        Ok(Self {
            auctioneer_guid,
            list_start_item,
            searched_name,
            minimum_level,
            maximum_level,
            auction_slot_id,
            auction_main_category,
            auction_sub_category,
            auction_quality,
            usable,
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
            // auctioneer_guid: Guid
            let auctioneer_guid = Guid::tokio_read(r).await?;

            // list_start_item: u32
            let list_start_item = crate::util::tokio_read_u32_le(r).await?;

            // searched_name: CString
            let searched_name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let searched_name = String::from_utf8(searched_name)?;

            // minimum_level: u8
            let minimum_level = crate::util::tokio_read_u8_le(r).await?;

            // maximum_level: u8
            let maximum_level = crate::util::tokio_read_u8_le(r).await?;

            // auction_slot_id: u32
            let auction_slot_id = crate::util::tokio_read_u32_le(r).await?;

            // auction_main_category: u32
            let auction_main_category = crate::util::tokio_read_u32_le(r).await?;

            // auction_sub_category: u32
            let auction_sub_category = crate::util::tokio_read_u32_le(r).await?;

            // auction_quality: u32
            let auction_quality = crate::util::tokio_read_u32_le(r).await?;

            // usable: u8
            let usable = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                auctioneer_guid,
                list_start_item,
                searched_name,
                minimum_level,
                maximum_level,
                auction_slot_id,
                auction_main_category,
                auction_sub_category,
                auction_quality,
                usable,
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
            // auctioneer_guid: Guid
            let auctioneer_guid = Guid::astd_read(r).await?;

            // list_start_item: u32
            let list_start_item = crate::util::astd_read_u32_le(r).await?;

            // searched_name: CString
            let searched_name = crate::util::astd_read_c_string_to_vec(r).await?;
            let searched_name = String::from_utf8(searched_name)?;

            // minimum_level: u8
            let minimum_level = crate::util::astd_read_u8_le(r).await?;

            // maximum_level: u8
            let maximum_level = crate::util::astd_read_u8_le(r).await?;

            // auction_slot_id: u32
            let auction_slot_id = crate::util::astd_read_u32_le(r).await?;

            // auction_main_category: u32
            let auction_main_category = crate::util::astd_read_u32_le(r).await?;

            // auction_sub_category: u32
            let auction_sub_category = crate::util::astd_read_u32_le(r).await?;

            // auction_quality: u32
            let auction_quality = crate::util::astd_read_u32_le(r).await?;

            // usable: u8
            let usable = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                auctioneer_guid,
                list_start_item,
                searched_name,
                minimum_level,
                maximum_level,
                auction_slot_id,
                auction_main_category,
                auction_sub_category,
                auction_quality,
                usable,
            })
        })
    }

}

impl CMSG_AUCTION_LIST_ITEMS {
    pub fn size(&self) -> usize {
        0
        + 8 // auctioneer_guid: Guid
        + 4 // list_start_item: u32
        + self.searched_name.len() + 1 // searched_name: CString
        + 1 // minimum_level: u8
        + 1 // maximum_level: u8
        + 4 // auction_slot_id: u32
        + 4 // auction_main_category: u32
        + 4 // auction_sub_category: u32
        + 4 // auction_quality: u32
        + 1 // usable: u8
    }
}

