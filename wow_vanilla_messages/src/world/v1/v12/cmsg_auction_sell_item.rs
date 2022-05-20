use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_AUCTION_SELL_ITEM {
    pub auctioneer_guid: Guid,
    pub object_guid: Guid,
    pub stack_size: u32,
    pub starting_bid: u32,
    pub buyout: u32,
    pub auction_duration_in_minutes: u32,
}

impl ClientMessageWrite for CMSG_AUCTION_SELL_ITEM {}

impl CMSG_AUCTION_SELL_ITEM {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 32], std::io::Error> {
        let mut array_w = [0u8; 32];
        let mut w = array_w.as_mut_slice();
        // auctioneer_guid: Guid
        w.write_all(&self.auctioneer_guid.guid().to_le_bytes())?;

        // object_guid: Guid
        w.write_all(&self.object_guid.guid().to_le_bytes())?;

        // stack_size: u32
        w.write_all(&self.stack_size.to_le_bytes())?;

        // starting_bid: u32
        w.write_all(&self.starting_bid.to_le_bytes())?;

        // buyout: u32
        w.write_all(&self.buyout.to_le_bytes())?;

        // auction_duration_in_minutes: u32
        w.write_all(&self.auction_duration_in_minutes.to_le_bytes())?;

        Ok(array_w)
    }
}

impl MessageBody for CMSG_AUCTION_SELL_ITEM {
    const OPCODE: u16 = 0x0256;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // auctioneer_guid: Guid
        let auctioneer_guid = Guid::read(r)?;

        // object_guid: Guid
        let object_guid = Guid::read(r)?;

        // stack_size: u32
        let stack_size = crate::util::read_u32_le(r)?;

        // starting_bid: u32
        let starting_bid = crate::util::read_u32_le(r)?;

        // buyout: u32
        let buyout = crate::util::read_u32_le(r)?;

        // auction_duration_in_minutes: u32
        let auction_duration_in_minutes = crate::util::read_u32_le(r)?;

        Ok(Self {
            auctioneer_guid,
            object_guid,
            stack_size,
            starting_bid,
            buyout,
            auction_duration_in_minutes,
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
            // auctioneer_guid: Guid
            let auctioneer_guid = Guid::tokio_read(r).await?;

            // object_guid: Guid
            let object_guid = Guid::tokio_read(r).await?;

            // stack_size: u32
            let stack_size = crate::util::tokio_read_u32_le(r).await?;

            // starting_bid: u32
            let starting_bid = crate::util::tokio_read_u32_le(r).await?;

            // buyout: u32
            let buyout = crate::util::tokio_read_u32_le(r).await?;

            // auction_duration_in_minutes: u32
            let auction_duration_in_minutes = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                auctioneer_guid,
                object_guid,
                stack_size,
                starting_bid,
                buyout,
                auction_duration_in_minutes,
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
            // auctioneer_guid: Guid
            let auctioneer_guid = Guid::astd_read(r).await?;

            // object_guid: Guid
            let object_guid = Guid::astd_read(r).await?;

            // stack_size: u32
            let stack_size = crate::util::astd_read_u32_le(r).await?;

            // starting_bid: u32
            let starting_bid = crate::util::astd_read_u32_le(r).await?;

            // buyout: u32
            let buyout = crate::util::astd_read_u32_le(r).await?;

            // auction_duration_in_minutes: u32
            let auction_duration_in_minutes = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                auctioneer_guid,
                object_guid,
                stack_size,
                starting_bid,
                buyout,
                auction_duration_in_minutes,
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

impl CMSG_AUCTION_SELL_ITEM {
    pub(crate) fn size() -> usize {
        0
        + 8 // auctioneer_guid: Guid
        + 8 // object_guid: Guid
        + 4 // stack_size: u32
        + 4 // starting_bid: u32
        + 4 // buyout: u32
        + 4 // auction_duration_in_minutes: u32
    }
}

