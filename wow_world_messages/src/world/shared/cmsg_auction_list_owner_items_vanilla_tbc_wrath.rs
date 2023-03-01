use crate:: {
    Guid,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_list_owner_items.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_list_owner_items.wowm#L3):
/// ```text
/// cmsg CMSG_AUCTION_LIST_OWNER_ITEMS = 0x0259 {
///     Guid auctioneer;
///     u32 list_from;
/// }
/// ```
pub struct CMSG_AUCTION_LIST_OWNER_ITEMS {
    pub auctioneer: Guid,
    pub list_from: u32,
}

impl crate::Message for CMSG_AUCTION_LIST_OWNER_ITEMS {
    const OPCODE: u32 = 0x0259;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // auctioneer: Guid
        w.write_all(&self.auctioneer.guid().to_le_bytes())?;

        // list_from: u32
        w.write_all(&self.list_from.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0259, size: body_size as u32 });
        }

        // auctioneer: Guid
        let auctioneer = Guid::read(&mut r)?;

        // list_from: u32
        let list_from = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            auctioneer,
            list_from,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_AUCTION_LIST_OWNER_ITEMS {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_AUCTION_LIST_OWNER_ITEMS {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_AUCTION_LIST_OWNER_ITEMS {}

