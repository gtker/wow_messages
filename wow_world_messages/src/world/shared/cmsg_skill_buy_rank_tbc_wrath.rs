use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_skill_buy_rank.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_skill_buy_rank.wowm#L1):
/// ```text
/// cmsg CMSG_SKILL_BUY_RANK = 0x0220 {
/// }
/// ```
pub struct CMSG_SKILL_BUY_RANK {
}

impl crate::Message for CMSG_SKILL_BUY_RANK {
    const OPCODE: u32 = 0x0220;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0220, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_SKILL_BUY_RANK {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_SKILL_BUY_RANK {}

