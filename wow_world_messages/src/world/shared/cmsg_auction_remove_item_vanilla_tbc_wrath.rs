use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_remove_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_remove_item.wowm#L3):
/// ```text
/// cmsg CMSG_AUCTION_REMOVE_ITEM = 0x0257 {
///     Guid auctioneer;
///     u32 auction_id;
/// }
/// ```
pub struct CMSG_AUCTION_REMOVE_ITEM {
    pub auctioneer: Guid,
    pub auction_id: u32,
}

impl crate::private::Sealed for CMSG_AUCTION_REMOVE_ITEM {}
impl crate::Message for CMSG_AUCTION_REMOVE_ITEM {
    const OPCODE: u32 = 0x0257;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // auctioneer: Guid
        w.write_all(&self.auctioneer.guid().to_le_bytes())?;

        // auction_id: u32
        w.write_all(&self.auction_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0257, size: body_size });
        }

        // auctioneer: Guid
        let auctioneer = Guid::read(&mut r)?;

        // auction_id: u32
        let auction_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            auctioneer,
            auction_id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_AUCTION_REMOVE_ITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_AUCTION_REMOVE_ITEM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_AUCTION_REMOVE_ITEM {}

