use std::convert::{TryFrom, TryInto};
use crate::helper::Guid;
use crate::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_owner_notification.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_owner_notification.wowm#L3):
/// ```text
/// smsg SMSG_AUCTION_OWNER_NOTIFICATION = 0x25F {
///     u32 auction_id;
///     u32 bid;
///     u32 auction_out_bid;
///     Guid bidder;
///     u32 item_entry;
///     u32 item_random_property_id;
/// }
/// ```
pub struct SMSG_AUCTION_OWNER_NOTIFICATION {
    pub auction_id: u32,
    pub bid: u32,
    pub auction_out_bid: u32,
    pub bidder: Guid,
    pub item_entry: u32,
    pub item_random_property_id: u32,
}

impl WorldServerMessageWrite for SMSG_AUCTION_OWNER_NOTIFICATION {
    const OPCODE: u16 = 0x25f;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_AUCTION_OWNER_NOTIFICATION {
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
}

impl ConstantSized for SMSG_AUCTION_OWNER_NOTIFICATION {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

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

