use crate::vanilla::Gold;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/trade/cmsg_set_trade_gold.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/trade/cmsg_set_trade_gold.wowm#L3):
/// ```text
/// cmsg CMSG_SET_TRADE_GOLD = 0x011F {
///     Gold gold;
/// }
/// ```
pub struct CMSG_SET_TRADE_GOLD {
    pub gold: Gold,
}

impl crate::Message for CMSG_SET_TRADE_GOLD {
    const OPCODE: u32 = 0x011f;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // gold: Gold
        w.write_all(u32::from(self.gold.as_int()).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x011F, size: body_size as u32 });
        }

        // gold: Gold
        let gold = Gold::new(crate::util::read_u32_le(r)?);

        Ok(Self {
            gold,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_SET_TRADE_GOLD {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_TRADE_GOLD {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_TRADE_GOLD {}

