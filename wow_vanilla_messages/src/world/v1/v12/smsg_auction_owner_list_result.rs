use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::AuctionListItem;
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
pub struct SMSG_AUCTION_OWNER_LIST_RESULT {
    pub auctions: Vec<AuctionListItem>,
    pub total_amount_of_auctions: u32,
}

impl ServerMessageWrite for SMSG_AUCTION_OWNER_LIST_RESULT {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_AUCTION_OWNER_LIST_RESULT {
    const OPCODE: u16 = 0x025d;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // count: u32
        let count = crate::util::read_u32_le(r)?;

        // auctions: AuctionListItem[count]
        let mut auctions = Vec::with_capacity(count as usize);
        for i in 0..count {
            auctions.push(AuctionListItem::read(r)?);
        }

        // total_amount_of_auctions: u32
        let total_amount_of_auctions = crate::util::read_u32_le(r)?;

        Ok(Self {
            auctions,
            total_amount_of_auctions,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // count: u32
        w.write_all(&(self.auctions.len() as u32).to_le_bytes())?;

        // auctions: AuctionListItem[count]
        for i in self.auctions.iter() {
            i.write(w)?;
        }

        // total_amount_of_auctions: u32
        w.write_all(&self.total_amount_of_auctions.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // count: u32
        let count = crate::util::tokio_read_u32_le(r).await?;

        // auctions: AuctionListItem[count]
        let mut auctions = Vec::with_capacity(count as usize);
        for i in 0..count {
            auctions.push(AuctionListItem::tokio_read(r).await?);
        }

        // total_amount_of_auctions: u32
        let total_amount_of_auctions = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            auctions,
            total_amount_of_auctions,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // count: u32
        w.write_all(&(self.auctions.len() as u32).to_le_bytes()).await?;

        // auctions: AuctionListItem[count]
        for i in self.auctions.iter() {
            i.tokio_write(w).await?;
        }

        // total_amount_of_auctions: u32
        w.write_all(&self.total_amount_of_auctions.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // count: u32
        let count = crate::util::astd_read_u32_le(r).await?;

        // auctions: AuctionListItem[count]
        let mut auctions = Vec::with_capacity(count as usize);
        for i in 0..count {
            auctions.push(AuctionListItem::astd_read(r).await?);
        }

        // total_amount_of_auctions: u32
        let total_amount_of_auctions = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            auctions,
            total_amount_of_auctions,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // count: u32
        w.write_all(&(self.auctions.len() as u32).to_le_bytes()).await?;

        // auctions: AuctionListItem[count]
        for i in self.auctions.iter() {
            i.astd_write(w).await?;
        }

        // total_amount_of_auctions: u32
        w.write_all(&self.total_amount_of_auctions.to_le_bytes()).await?;

        Ok(())
    }
}

impl VariableSized for SMSG_AUCTION_OWNER_LIST_RESULT {
    fn size(&self) -> usize {
        4 // count: u32
        + self.auctions.iter().fold(0, |acc, x| acc + AuctionListItem::size()) // auctions: AuctionListItem[count]
        + 4 // total_amount_of_auctions: u32
    }
}

impl MaximumPossibleSized for SMSG_AUCTION_OWNER_LIST_RESULT {
    fn maximum_possible_size() -> usize {
        4 // count: u32
        + 4294967295 * AuctionListItem::maximum_possible_size() // auctions: AuctionListItem[count]
        + 4 // total_amount_of_auctions: u32
    }
}

