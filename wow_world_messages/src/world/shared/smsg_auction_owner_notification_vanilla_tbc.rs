use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// vmangos/cmangos/mangoszero: this message causes on client to display: 'Your auction sold'
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_owner_notification.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_owner_notification.wowm#L1):
/// ```text
/// smsg SMSG_AUCTION_OWNER_NOTIFICATION = 0x025F {
///     u32 auction_id;
///     u32 bid;
///     u32 auction_out_bid;
///     Guid bidder;
///     u32 item;
///     u32 item_random_property_id;
/// }
/// ```
pub struct SMSG_AUCTION_OWNER_NOTIFICATION {
    pub auction_id: u32,
    /// vmangos/cmangos/mangoszero: if 0, client shows ERR_AUCTION_EXPIRED_S, else ERR_AUCTION_SOLD_S (works only when guid==0)
    ///
    pub bid: u32,
    pub auction_out_bid: u32,
    pub bidder: Guid,
    pub item: u32,
    pub item_random_property_id: u32,
}

impl crate::Message for SMSG_AUCTION_OWNER_NOTIFICATION {
    const OPCODE: u32 = 0x025f;

    fn size_without_header(&self) -> u32 {
        28
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // auction_id: u32
        w.write_all(&self.auction_id.to_le_bytes())?;

        // bid: u32
        w.write_all(&self.bid.to_le_bytes())?;

        // auction_out_bid: u32
        w.write_all(&self.auction_out_bid.to_le_bytes())?;

        // bidder: Guid
        w.write_all(&self.bidder.guid().to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 28 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x025F, size: body_size as u32 });
        }

        // auction_id: u32
        let auction_id = crate::util::read_u32_le(r)?;

        // bid: u32
        let bid = crate::util::read_u32_le(r)?;

        // auction_out_bid: u32
        let auction_out_bid = crate::util::read_u32_le(r)?;

        // bidder: Guid
        let bidder = Guid::read(r)?;

        // item: u32
        let item = crate::util::read_u32_le(r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            auction_id,
            bid,
            auction_out_bid,
            bidder,
            item,
            item_random_property_id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_AUCTION_OWNER_NOTIFICATION {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_AUCTION_OWNER_NOTIFICATION {}

