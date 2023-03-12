use std::io::{Read, Write};
use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_sell_item.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_sell_item.wowm#L1):
/// ```text
/// cmsg CMSG_AUCTION_SELL_ITEM = 0x0256 {
///     Guid auctioneer;
///     Guid item;
///     u32 starting_bid;
///     u32 buyout;
///     u32 auction_duration_in_minutes;
/// }
/// ```
pub struct CMSG_AUCTION_SELL_ITEM {
    pub auctioneer: Guid,
    pub item: Guid,
    pub starting_bid: u32,
    pub buyout: u32,
    pub auction_duration_in_minutes: u32,
}

impl crate::Message for CMSG_AUCTION_SELL_ITEM {
    const OPCODE: u32 = 0x0256;

    fn size_without_header(&self) -> u32 {
        28
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // auctioneer: Guid
        w.write_all(&self.auctioneer.guid().to_le_bytes())?;

        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // starting_bid: u32
        w.write_all(&self.starting_bid.to_le_bytes())?;

        // buyout: u32
        w.write_all(&self.buyout.to_le_bytes())?;

        // auction_duration_in_minutes: u32
        w.write_all(&self.auction_duration_in_minutes.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 28 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0256, size: body_size as u32 });
        }

        // auctioneer: Guid
        let auctioneer = Guid::read(&mut r)?;

        // item: Guid
        let item = Guid::read(&mut r)?;

        // starting_bid: u32
        let starting_bid = crate::util::read_u32_le(&mut r)?;

        // buyout: u32
        let buyout = crate::util::read_u32_le(&mut r)?;

        // auction_duration_in_minutes: u32
        let auction_duration_in_minutes = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            auctioneer,
            item,
            starting_bid,
            buyout,
            auction_duration_in_minutes,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_AUCTION_SELL_ITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_AUCTION_SELL_ITEM {}

