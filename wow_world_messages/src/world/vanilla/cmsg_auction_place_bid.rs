use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_place_bid.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_place_bid.wowm#L3):
/// ```text
/// cmsg CMSG_AUCTION_PLACE_BID = 0x025A {
///     Guid auctioneer_guid;
///     u32 auction_id;
///     u32 price;
/// }
/// ```
pub struct CMSG_AUCTION_PLACE_BID {
    pub auctioneer_guid: Guid,
    pub auction_id: u32,
    pub price: u32,
}

impl crate::Message for CMSG_AUCTION_PLACE_BID {
    const OPCODE: u32 = 0x025a;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // auctioneer_guid: Guid
        w.write_all(&self.auctioneer_guid.guid().to_le_bytes())?;

        // auction_id: u32
        w.write_all(&self.auction_id.to_le_bytes())?;

        // price: u32
        w.write_all(&self.price.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // auctioneer_guid: Guid
        let auctioneer_guid = Guid::read(r)?;

        // auction_id: u32
        let auction_id = crate::util::read_u32_le(r)?;

        // price: u32
        let price = crate::util::read_u32_le(r)?;

        Ok(Self {
            auctioneer_guid,
            auction_id,
            price,
        })
    }

}
#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ClientMessage for CMSG_AUCTION_PLACE_BID {}

