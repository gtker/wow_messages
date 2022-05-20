use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_AUCTION_BIDDER_NOTIFICATION {
    pub auction_house_id: u32,
    pub auction_id: u32,
    pub bidder: Guid,
    pub won: u32,
    pub out_bid: u32,
    pub item_template: u32,
    pub item_random_property_id: u32,
}

impl ServerMessageWrite for SMSG_AUCTION_BIDDER_NOTIFICATION {}

impl SMSG_AUCTION_BIDDER_NOTIFICATION {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 32], std::io::Error> {
        let mut array_w = [0u8; 32];
        let mut w = array_w.as_mut_slice();
        // auction_house_id: u32
        w.write_all(&self.auction_house_id.to_le_bytes())?;

        // auction_id: u32
        w.write_all(&self.auction_id.to_le_bytes())?;

        // bidder: Guid
        w.write_all(&self.bidder.guid().to_le_bytes())?;

        // won: u32
        w.write_all(&self.won.to_le_bytes())?;

        // out_bid: u32
        w.write_all(&self.out_bid.to_le_bytes())?;

        // item_template: u32
        w.write_all(&self.item_template.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        Ok(array_w)
    }
}

impl MessageBody for SMSG_AUCTION_BIDDER_NOTIFICATION {
    const OPCODE: u16 = 0x025e;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // auction_house_id: u32
        let auction_house_id = crate::util::read_u32_le(r)?;

        // auction_id: u32
        let auction_id = crate::util::read_u32_le(r)?;

        // bidder: Guid
        let bidder = Guid::read(r)?;

        // won: u32
        let won = crate::util::read_u32_le(r)?;

        // out_bid: u32
        let out_bid = crate::util::read_u32_le(r)?;

        // item_template: u32
        let item_template = crate::util::read_u32_le(r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            auction_house_id,
            auction_id,
            bidder,
            won,
            out_bid,
            item_template,
            item_random_property_id,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let inner = self.as_bytes()?;
        w.write_all(&inner)
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
            // auction_house_id: u32
            let auction_house_id = crate::util::tokio_read_u32_le(r).await?;

            // auction_id: u32
            let auction_id = crate::util::tokio_read_u32_le(r).await?;

            // bidder: Guid
            let bidder = Guid::tokio_read(r).await?;

            // won: u32
            let won = crate::util::tokio_read_u32_le(r).await?;

            // out_bid: u32
            let out_bid = crate::util::tokio_read_u32_le(r).await?;

            // item_template: u32
            let item_template = crate::util::tokio_read_u32_le(r).await?;

            // item_random_property_id: u32
            let item_random_property_id = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                auction_house_id,
                auction_id,
                bidder,
                won,
                out_bid,
                item_template,
                item_random_property_id,
            })
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
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
            // auction_house_id: u32
            let auction_house_id = crate::util::astd_read_u32_le(r).await?;

            // auction_id: u32
            let auction_id = crate::util::astd_read_u32_le(r).await?;

            // bidder: Guid
            let bidder = Guid::astd_read(r).await?;

            // won: u32
            let won = crate::util::astd_read_u32_le(r).await?;

            // out_bid: u32
            let out_bid = crate::util::astd_read_u32_le(r).await?;

            // item_template: u32
            let item_template = crate::util::astd_read_u32_le(r).await?;

            // item_random_property_id: u32
            let item_random_property_id = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                auction_house_id,
                auction_id,
                bidder,
                won,
                out_bid,
                item_template,
                item_random_property_id,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

}

impl SMSG_AUCTION_BIDDER_NOTIFICATION {
    pub(crate) fn size() -> usize {
        0
        + 4 // auction_house_id: u32
        + 4 // auction_id: u32
        + 8 // bidder: Guid
        + 4 // won: u32
        + 4 // out_bid: u32
        + 4 // item_template: u32
        + 4 // item_random_property_id: u32
    }
}

