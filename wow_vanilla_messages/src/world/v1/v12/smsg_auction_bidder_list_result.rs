use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::AuctionListItem;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_AUCTION_BIDDER_LIST_RESULT {
    pub auctions: Vec<AuctionListItem>,
    pub total_amount_of_auctions: u32,
}

impl ServerMessageWrite for SMSG_AUCTION_BIDDER_LIST_RESULT {}

impl MessageBody for SMSG_AUCTION_BIDDER_LIST_RESULT {
    const OPCODE: u16 = 0x0265;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
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

    #[cfg(feature = "sync")]
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
        })
    }

    #[cfg(feature = "async_tokio")]
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
            // count: u32
            w.write_all(&(self.auctions.len() as u32).to_le_bytes()).await?;

            // auctions: AuctionListItem[count]
            for i in self.auctions.iter() {
                i.tokio_write(w).await?;
            }

            // total_amount_of_auctions: u32
            w.write_all(&self.total_amount_of_auctions.to_le_bytes()).await?;

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
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
        })
    }

    #[cfg(feature = "async_std")]
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
            // count: u32
            w.write_all(&(self.auctions.len() as u32).to_le_bytes()).await?;

            // auctions: AuctionListItem[count]
            for i in self.auctions.iter() {
                i.astd_write(w).await?;
            }

            // total_amount_of_auctions: u32
            w.write_all(&self.total_amount_of_auctions.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl VariableSized for SMSG_AUCTION_BIDDER_LIST_RESULT {
    fn size(&self) -> usize {
        0
        + 4 // count: u32
        + self.auctions.iter().fold(0, |acc, x| acc + AuctionListItem::size()) // auctions: AuctionListItem[count]
        + 4 // total_amount_of_auctions: u32
    }
}

impl MaximumPossibleSized for SMSG_AUCTION_BIDDER_LIST_RESULT {
    fn maximum_possible_size() -> usize {
        65535 // Capped at u16::MAX due to size field.
    }
}

