use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/msg/msg_auction_hello_server.wowm:20`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/msg/msg_auction_hello_server.wowm#L20):
/// ```text
/// smsg MSG_AUCTION_HELLO_Server = 0x0255 {
///     Guid auctioneer;
///     u32 auction_house_id;
///     Bool auction_house_enabled;
/// }
/// ```
pub struct MSG_AUCTION_HELLO_Server {
    pub auctioneer: Guid,
    pub auction_house_id: u32,
    pub auction_house_enabled: bool,
}

impl crate::Message for MSG_AUCTION_HELLO_Server {
    const OPCODE: u32 = 0x0255;

    fn size_without_header(&self) -> u32 {
        13
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // auctioneer: Guid
        w.write_all(&self.auctioneer.guid().to_le_bytes())?;

        // auction_house_id: u32
        w.write_all(&self.auction_house_id.to_le_bytes())?;

        // auction_house_enabled: Bool
        w.write_all(u8::from(self.auction_house_enabled).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 13 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // auctioneer: Guid
        let auctioneer = Guid::read(r)?;

        // auction_house_id: u32
        let auction_house_id = crate::util::read_u32_le(r)?;

        // auction_house_enabled: Bool
        let auction_house_enabled = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            auctioneer,
            auction_house_id,
            auction_house_enabled,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for MSG_AUCTION_HELLO_Server {}

