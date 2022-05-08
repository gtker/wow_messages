use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

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
        // auctioneer_guid: Guid
        self.auctioneer_guid.write(w)?;

        // object_guid: Guid
        self.object_guid.write(w)?;

        // stack_size: u32
        w.write_all(&self.stack_size.to_le_bytes())?;

        // starting_bid: u32
        w.write_all(&self.starting_bid.to_le_bytes())?;

        // buyout: u32
        w.write_all(&self.buyout.to_le_bytes())?;

        // auction_duration_in_minutes: u32
        w.write_all(&self.auction_duration_in_minutes.to_le_bytes())?;

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
            // auctioneer_guid: Guid
            self.auctioneer_guid.tokio_write(w).await?;

            // object_guid: Guid
            self.object_guid.tokio_write(w).await?;

            // stack_size: u32
            w.write_all(&self.stack_size.to_le_bytes()).await?;

            // starting_bid: u32
            w.write_all(&self.starting_bid.to_le_bytes()).await?;

            // buyout: u32
            w.write_all(&self.buyout.to_le_bytes()).await?;

            // auction_duration_in_minutes: u32
            w.write_all(&self.auction_duration_in_minutes.to_le_bytes()).await?;

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
            // auctioneer_guid: Guid
            self.auctioneer_guid.astd_write(w).await?;

            // object_guid: Guid
            self.object_guid.astd_write(w).await?;

            // stack_size: u32
            w.write_all(&self.stack_size.to_le_bytes()).await?;

            // starting_bid: u32
            w.write_all(&self.starting_bid.to_le_bytes()).await?;

            // buyout: u32
            w.write_all(&self.buyout.to_le_bytes()).await?;

            // auction_duration_in_minutes: u32
            w.write_all(&self.auction_duration_in_minutes.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for CMSG_AUCTION_SELL_ITEM {}

impl MaximumPossibleSized for CMSG_AUCTION_SELL_ITEM {
    fn maximum_possible_size() -> usize {
        0
        + 8 // auctioneer_guid: Guid
        + 8 // object_guid: Guid
        + 4 // stack_size: u32
        + 4 // starting_bid: u32
        + 4 // buyout: u32
        + 4 // auction_duration_in_minutes: u32
    }
}

