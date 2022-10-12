use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_sell_item.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_sell_item.wowm#L11):
/// ```text
/// cmsg CMSG_AUCTION_SELL_ITEM = 0x0256 {
///     Guid auctioneer;
///     u32 unknown1;
///     Guid item;
///     u32 unknown2;
///     u32 starting_bid;
///     u32 buyout;
///     u32 auction_duration_in_minutes;
/// }
/// ```
pub struct CMSG_AUCTION_SELL_ITEM {
    pub auctioneer: Guid,
    pub unknown1: u32,
    pub item: Guid,
    pub unknown2: u32,
    pub starting_bid: u32,
    pub buyout: u32,
    pub auction_duration_in_minutes: u32,
}

impl crate::Message for CMSG_AUCTION_SELL_ITEM {
    const OPCODE: u32 = 0x0256;

    fn size_without_header(&self) -> u32 {
        36
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // auctioneer: Guid
        w.write_all(&self.auctioneer.guid().to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // starting_bid: u32
        w.write_all(&self.starting_bid.to_le_bytes())?;

        // buyout: u32
        w.write_all(&self.buyout.to_le_bytes())?;

        // auction_duration_in_minutes: u32
        w.write_all(&self.auction_duration_in_minutes.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 36 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // auctioneer: Guid
        let auctioneer = Guid::read(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // item: Guid
        let item = Guid::read(r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(r)?;

        // starting_bid: u32
        let starting_bid = crate::util::read_u32_le(r)?;

        // buyout: u32
        let buyout = crate::util::read_u32_le(r)?;

        // auction_duration_in_minutes: u32
        let auction_duration_in_minutes = crate::util::read_u32_le(r)?;

        Ok(Self {
            auctioneer,
            unknown1,
            item,
            unknown2,
            starting_bid,
            buyout,
            auction_duration_in_minutes,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_AUCTION_SELL_ITEM {}

