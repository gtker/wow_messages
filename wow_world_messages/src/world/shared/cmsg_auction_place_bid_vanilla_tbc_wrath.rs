use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::gold_vanilla_tbc_wrath::Gold;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_place_bid.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_place_bid.wowm#L3):
/// ```text
/// cmsg CMSG_AUCTION_PLACE_BID = 0x025A {
///     Guid auctioneer;
///     u32 auction_id;
///     Gold price;
/// }
/// ```
pub struct CMSG_AUCTION_PLACE_BID {
    pub auctioneer: Guid,
    pub auction_id: u32,
    pub price: Gold,
}

impl crate::private::Sealed for CMSG_AUCTION_PLACE_BID {}
impl crate::Message for CMSG_AUCTION_PLACE_BID {
    const OPCODE: u32 = 0x025a;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // auctioneer: Guid
        w.write_all(&self.auctioneer.guid().to_le_bytes())?;

        // auction_id: u32
        w.write_all(&self.auction_id.to_le_bytes())?;

        // price: Gold
        w.write_all((self.price.as_int()).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x025A, size: body_size });
        }

        // auctioneer: Guid
        let auctioneer = Guid::read(&mut r)?;

        // auction_id: u32
        let auction_id = crate::util::read_u32_le(&mut r)?;

        // price: Gold
        let price = Gold::new(crate::util::read_u32_le(&mut r)?);

        Ok(Self {
            auctioneer,
            auction_id,
            price,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_AUCTION_PLACE_BID {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_AUCTION_PLACE_BID {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_AUCTION_PLACE_BID {}

