use crate:: {
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/trade/cmsg_unaccept_trade.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/trade/cmsg_unaccept_trade.wowm#L3):
/// ```text
/// cmsg CMSG_UNACCEPT_TRADE = 0x011B {
/// }
/// ```
pub struct CMSG_UNACCEPT_TRADE {
}

impl crate::Message for CMSG_UNACCEPT_TRADE {
    const OPCODE: u32 = 0x011b;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x011B, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_UNACCEPT_TRADE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_UNACCEPT_TRADE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_UNACCEPT_TRADE {}

