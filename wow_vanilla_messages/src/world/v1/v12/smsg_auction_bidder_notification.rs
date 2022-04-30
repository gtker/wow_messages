use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

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

impl MessageBody for SMSG_AUCTION_BIDDER_NOTIFICATION {
    const OPCODE: u16 = 0x025e;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

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

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // auction_house_id: u32
        w.write_all(&self.auction_house_id.to_le_bytes())?;

        // auction_id: u32
        w.write_all(&self.auction_id.to_le_bytes())?;

        // bidder: Guid
        self.bidder.write(w)?;

        // won: u32
        w.write_all(&self.won.to_le_bytes())?;

        // out_bid: u32
        w.write_all(&self.out_bid.to_le_bytes())?;

        // item_template: u32
        w.write_all(&self.item_template.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_AUCTION_BIDDER_NOTIFICATION {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_AUCTION_BIDDER_NOTIFICATION {
    fn maximum_possible_size() -> usize {
        4 // auction_house_id: u32
        + 4 // auction_id: u32
        + 8 // bidder: Guid
        + 4 // won: u32
        + 4 // out_bid: u32
        + 4 // item_template: u32
        + 4 // item_random_property_id: u32
    }
}

