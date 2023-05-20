use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_list_pending_sales.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_list_pending_sales.wowm#L1):
/// ```text
/// cmsg CMSG_AUCTION_LIST_PENDING_SALES = 0x048F {
///     Guid auctioneer;
/// }
/// ```
pub struct CMSG_AUCTION_LIST_PENDING_SALES {
    pub auctioneer: Guid,
}

impl crate::private::Sealed for CMSG_AUCTION_LIST_PENDING_SALES {}
impl crate::Message for CMSG_AUCTION_LIST_PENDING_SALES {
    const OPCODE: u32 = 0x048f;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // auctioneer: Guid
        w.write_all(&self.auctioneer.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x048F, size: body_size });
        }

        // auctioneer: Guid
        let auctioneer = Guid::read(&mut r)?;

        Ok(Self {
            auctioneer,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_AUCTION_LIST_PENDING_SALES {}

