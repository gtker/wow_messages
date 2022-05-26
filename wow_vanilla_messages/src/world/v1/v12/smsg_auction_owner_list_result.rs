use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::AuctionListItem;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_AUCTION_OWNER_LIST_RESULT {
    pub auctions: Vec<AuctionListItem>,
    pub total_amount_of_auctions: u32,
}

impl ServerMessage for SMSG_AUCTION_OWNER_LIST_RESULT {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // count: u32
        w.write_all(&(self.auctions.len() as u32).to_le_bytes())?;

        // auctions: AuctionListItem[count]
        for i in self.auctions.iter() {
            i.as_bytes(w)?;
        }

        // total_amount_of_auctions: u32
        w.write_all(&self.total_amount_of_auctions.to_le_bytes())?;

        Ok(())
    }
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

}

impl SMSG_AUCTION_OWNER_LIST_RESULT {
    pub fn size(&self) -> usize {
        0
        + 4 // count: u32
        + self.auctions.len() * 64 // auctions: AuctionListItem[count]
        + 4 // total_amount_of_auctions: u32
    }
}

