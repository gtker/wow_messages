use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
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

impl ClientMessage for CMSG_AUCTION_PLACE_BID {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // auctioneer_guid: Guid
        w.write_all(&self.auctioneer_guid.guid().to_le_bytes())?;

        // auction_id: u32
        w.write_all(&self.auction_id.to_le_bytes())?;

        // price: u32
        w.write_all(&self.price.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x025a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        16
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

