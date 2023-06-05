use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::auction_house_vanilla_tbc_wrath::AuctionHouse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_notification.wowm:27`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_notification.wowm#L27):
/// ```text
/// smsg SMSG_AUCTION_BIDDER_NOTIFICATION = 0x025E {
///     AuctionHouse auction_house;
///     u32 auction_id;
///     Guid bidder;
///     u32 won;
///     u32 out_bid;
///     u32 item_template;
///     u32 item_random_property_id;
/// }
/// ```
pub struct SMSG_AUCTION_BIDDER_NOTIFICATION {
    pub auction_house: AuctionHouse,
    pub auction_id: u32,
    pub bidder: Guid,
    /// vmangos/cmangos: if 0, client shows ERR_AUCTION_WON_S, else ERR_AUCTION_OUTBID_S
    ///
    pub won: u32,
    pub out_bid: u32,
    pub item_template: u32,
    pub item_random_property_id: u32,
}

impl crate::private::Sealed for SMSG_AUCTION_BIDDER_NOTIFICATION {}
impl crate::Message for SMSG_AUCTION_BIDDER_NOTIFICATION {
    const OPCODE: u32 = 0x025e;

    fn size_without_header(&self) -> u32 {
        32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // auction_house: AuctionHouse
        w.write_all(&(self.auction_house.as_int().to_le_bytes()))?;

        // auction_id: u32
        w.write_all(&self.auction_id.to_le_bytes())?;

        // bidder: Guid
        w.write_all(&self.bidder.guid().to_le_bytes())?;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 32 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x025E, size: body_size });
        }

        // auction_house: AuctionHouse
        let auction_house = crate::util::read_u32_le(&mut r)?.try_into()?;

        // auction_id: u32
        let auction_id = crate::util::read_u32_le(&mut r)?;

        // bidder: Guid
        let bidder = crate::util::read_guid(&mut r)?;

        // won: u32
        let won = crate::util::read_u32_le(&mut r)?;

        // out_bid: u32
        let out_bid = crate::util::read_u32_le(&mut r)?;

        // item_template: u32
        let item_template = crate::util::read_u32_le(&mut r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            auction_house,
            auction_id,
            bidder,
            won,
            out_bid,
            item_template,
            item_random_property_id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_AUCTION_BIDDER_NOTIFICATION {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_AUCTION_BIDDER_NOTIFICATION {}

