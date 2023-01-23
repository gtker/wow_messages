use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/trade/cmsg_cancel_trade.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/trade/cmsg_cancel_trade.wowm#L3):
/// ```text
/// cmsg CMSG_CANCEL_TRADE = 0x011C {
/// }
/// ```
pub struct CMSG_CANCEL_TRADE {
}

impl crate::Message for CMSG_CANCEL_TRADE {
    const OPCODE: u32 = 0x011c;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x011C, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_CANCEL_TRADE {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_CANCEL_TRADE {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_CANCEL_TRADE {}

