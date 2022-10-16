use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_chat_wrong_faction.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_chat_wrong_faction.wowm#L3):
/// ```text
/// smsg SMSG_CHAT_WRONG_FACTION = 0x0219 {
/// }
/// ```
pub struct SMSG_CHAT_WRONG_FACTION {
}

impl crate::Message for SMSG_CHAT_WRONG_FACTION {
    const OPCODE: u32 = 0x0219;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_CHAT_WRONG_FACTION {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_CHAT_WRONG_FACTION {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_CHAT_WRONG_FACTION {}

