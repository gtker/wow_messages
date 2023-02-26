use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/cmsg_loot_money.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/cmsg_loot_money.wowm#L3):
/// ```text
/// cmsg CMSG_LOOT_MONEY = 0x015E {
/// }
/// ```
pub struct CMSG_LOOT_MONEY {
}

impl crate::Message for CMSG_LOOT_MONEY {
    const OPCODE: u32 = 0x015e;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x015E, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_LOOT_MONEY {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_LOOT_MONEY {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_LOOT_MONEY {}

