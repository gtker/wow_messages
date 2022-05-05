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
pub struct SMSG_AUCTION_OWNER_NOTIFICATION {
    pub auction_id: u32,
    pub bid: u32,
    pub auction_out_bid: u32,
    pub bidder: Guid,
    pub item_entry: u32,
    pub item_random_property_id: u32,
}

impl ServerMessageWrite for SMSG_AUCTION_OWNER_NOTIFICATION {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_AUCTION_OWNER_NOTIFICATION {
    const OPCODE: u16 = 0x025f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // auction_id: u32
        let auction_id = crate::util::read_u32_le(r)?;

        // bid: u32
        let bid = crate::util::read_u32_le(r)?;

        // auction_out_bid: u32
        let auction_out_bid = crate::util::read_u32_le(r)?;

        // bidder: Guid
        let bidder = Guid::read(r)?;

        // item_entry: u32
        let item_entry = crate::util::read_u32_le(r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            auction_id,
            bid,
            auction_out_bid,
            bidder,
            item_entry,
            item_random_property_id,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // auction_id: u32
        w.write_all(&self.auction_id.to_le_bytes())?;

        // bid: u32
        w.write_all(&self.bid.to_le_bytes())?;

        // auction_out_bid: u32
        w.write_all(&self.auction_out_bid.to_le_bytes())?;

        // bidder: Guid
        self.bidder.write(w)?;

        // item_entry: u32
        w.write_all(&self.item_entry.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // auction_id: u32
        let auction_id = crate::util::tokio_read_u32_le(r).await?;

        // bid: u32
        let bid = crate::util::tokio_read_u32_le(r).await?;

        // auction_out_bid: u32
        let auction_out_bid = crate::util::tokio_read_u32_le(r).await?;

        // bidder: Guid
        let bidder = Guid::tokio_read(r).await?;

        // item_entry: u32
        let item_entry = crate::util::tokio_read_u32_le(r).await?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            auction_id,
            bid,
            auction_out_bid,
            bidder,
            item_entry,
            item_random_property_id,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // auction_id: u32
        w.write_all(&self.auction_id.to_le_bytes()).await?;

        // bid: u32
        w.write_all(&self.bid.to_le_bytes()).await?;

        // auction_out_bid: u32
        w.write_all(&self.auction_out_bid.to_le_bytes()).await?;

        // bidder: Guid
        self.bidder.tokio_write(w).await?;

        // item_entry: u32
        w.write_all(&self.item_entry.to_le_bytes()).await?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // auction_id: u32
        let auction_id = crate::util::astd_read_u32_le(r).await?;

        // bid: u32
        let bid = crate::util::astd_read_u32_le(r).await?;

        // auction_out_bid: u32
        let auction_out_bid = crate::util::astd_read_u32_le(r).await?;

        // bidder: Guid
        let bidder = Guid::astd_read(r).await?;

        // item_entry: u32
        let item_entry = crate::util::astd_read_u32_le(r).await?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            auction_id,
            bid,
            auction_out_bid,
            bidder,
            item_entry,
            item_random_property_id,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // auction_id: u32
        w.write_all(&self.auction_id.to_le_bytes()).await?;

        // bid: u32
        w.write_all(&self.bid.to_le_bytes()).await?;

        // auction_out_bid: u32
        w.write_all(&self.auction_out_bid.to_le_bytes()).await?;

        // bidder: Guid
        self.bidder.astd_write(w).await?;

        // item_entry: u32
        w.write_all(&self.item_entry.to_le_bytes()).await?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_AUCTION_OWNER_NOTIFICATION {}

impl MaximumPossibleSized for SMSG_AUCTION_OWNER_NOTIFICATION {
    fn maximum_possible_size() -> usize {
        4 // auction_id: u32
        + 4 // bid: u32
        + 4 // auction_out_bid: u32
        + 8 // bidder: Guid
        + 4 // item_entry: u32
        + 4 // item_random_property_id: u32
    }
}

