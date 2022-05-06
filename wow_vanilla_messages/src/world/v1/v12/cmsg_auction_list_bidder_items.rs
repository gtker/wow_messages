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
pub struct CMSG_AUCTION_LIST_BIDDER_ITEMS {
    pub auctioneer: Guid,
    pub start_from_page: u32,
    pub outbid_item_ids: Vec<u32>,
}

impl ClientMessageWrite for CMSG_AUCTION_LIST_BIDDER_ITEMS {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_AUCTION_LIST_BIDDER_ITEMS {
    const OPCODE: u16 = 0x0264;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // auctioneer: Guid
        let auctioneer = Guid::read(r)?;

        // start_from_page: u32
        let start_from_page = crate::util::read_u32_le(r)?;

        // amount_of_outbidded_items: u32
        let amount_of_outbidded_items = crate::util::read_u32_le(r)?;

        // outbid_item_ids: u32[amount_of_outbidded_items]
        let mut outbid_item_ids = Vec::with_capacity(amount_of_outbidded_items as usize);
        for i in 0..amount_of_outbidded_items {
            outbid_item_ids.push(crate::util::read_u32_le(r)?);
        }

        Ok(Self {
            auctioneer,
            start_from_page,
            outbid_item_ids,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // auctioneer: Guid
        self.auctioneer.write(w)?;

        // start_from_page: u32
        w.write_all(&self.start_from_page.to_le_bytes())?;

        // amount_of_outbidded_items: u32
        w.write_all(&(self.outbid_item_ids.len() as u32).to_le_bytes())?;

        // outbid_item_ids: u32[amount_of_outbidded_items]
        for i in self.outbid_item_ids.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // auctioneer: Guid
        let auctioneer = Guid::tokio_read(r).await?;

        // start_from_page: u32
        let start_from_page = crate::util::tokio_read_u32_le(r).await?;

        // amount_of_outbidded_items: u32
        let amount_of_outbidded_items = crate::util::tokio_read_u32_le(r).await?;

        // outbid_item_ids: u32[amount_of_outbidded_items]
        let mut outbid_item_ids = Vec::with_capacity(amount_of_outbidded_items as usize);
        for i in 0..amount_of_outbidded_items {
            outbid_item_ids.push(crate::util::tokio_read_u32_le(r).await?);
        }

        Ok(Self {
            auctioneer,
            start_from_page,
            outbid_item_ids,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // auctioneer: Guid
        self.auctioneer.tokio_write(w).await?;

        // start_from_page: u32
        w.write_all(&self.start_from_page.to_le_bytes()).await?;

        // amount_of_outbidded_items: u32
        w.write_all(&(self.outbid_item_ids.len() as u32).to_le_bytes()).await?;

        // outbid_item_ids: u32[amount_of_outbidded_items]
        for i in self.outbid_item_ids.iter() {
            w.write_all(&i.to_le_bytes()).await?;
        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // auctioneer: Guid
        let auctioneer = Guid::astd_read(r).await?;

        // start_from_page: u32
        let start_from_page = crate::util::astd_read_u32_le(r).await?;

        // amount_of_outbidded_items: u32
        let amount_of_outbidded_items = crate::util::astd_read_u32_le(r).await?;

        // outbid_item_ids: u32[amount_of_outbidded_items]
        let mut outbid_item_ids = Vec::with_capacity(amount_of_outbidded_items as usize);
        for i in 0..amount_of_outbidded_items {
            outbid_item_ids.push(crate::util::astd_read_u32_le(r).await?);
        }

        Ok(Self {
            auctioneer,
            start_from_page,
            outbid_item_ids,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // auctioneer: Guid
        self.auctioneer.astd_write(w).await?;

        // start_from_page: u32
        w.write_all(&self.start_from_page.to_le_bytes()).await?;

        // amount_of_outbidded_items: u32
        w.write_all(&(self.outbid_item_ids.len() as u32).to_le_bytes()).await?;

        // outbid_item_ids: u32[amount_of_outbidded_items]
        for i in self.outbid_item_ids.iter() {
            w.write_all(&i.to_le_bytes()).await?;
        }

        Ok(())
    }

}

impl VariableSized for CMSG_AUCTION_LIST_BIDDER_ITEMS {
    fn size(&self) -> usize {
        0
        + 8 // auctioneer: Guid
        + 4 // start_from_page: u32
        + 4 // amount_of_outbidded_items: u32
        + self.outbid_item_ids.len() * core::mem::size_of::<u32>() // outbid_item_ids: u32[amount_of_outbidded_items]
    }
}

impl MaximumPossibleSized for CMSG_AUCTION_LIST_BIDDER_ITEMS {
    fn maximum_possible_size() -> usize {
        65535 // Capped at u16::MAX due to size field.
    }
}

