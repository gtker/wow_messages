use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

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

impl SMSG_AUCTION_OWNER_NOTIFICATION {
    pub(crate) fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // auction_id: u32
        w.write_all(&self.auction_id.to_le_bytes())?;

        // bid: u32
        w.write_all(&self.bid.to_le_bytes())?;

        // auction_out_bid: u32
        w.write_all(&self.auction_out_bid.to_le_bytes())?;

        // bidder: Guid
        w.write_all(&self.bidder.guid().to_le_bytes())?;

        // item_entry: u32
        w.write_all(&self.item_entry.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        Ok(())
    }
}

impl ServerMessage for SMSG_AUCTION_OWNER_NOTIFICATION {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // auction_id: u32
        w.write_all(&self.auction_id.to_le_bytes())?;

        // bid: u32
        w.write_all(&self.bid.to_le_bytes())?;

        // auction_out_bid: u32
        w.write_all(&self.auction_out_bid.to_le_bytes())?;

        // bidder: Guid
        w.write_all(&self.bidder.guid().to_le_bytes())?;

        // item_entry: u32
        w.write_all(&self.item_entry.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x025f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        28
    }

    type Error = std::io::Error;

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

}

