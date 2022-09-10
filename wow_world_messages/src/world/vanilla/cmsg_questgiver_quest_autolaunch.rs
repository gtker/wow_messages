use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/cmsg_questgiver_quest_autolaunch.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/cmsg_questgiver_quest_autolaunch.wowm#L3):
/// ```text
/// cmsg CMSG_QUESTGIVER_QUEST_AUTOLAUNCH = 0x0187 {
/// }
/// ```
pub struct CMSG_QUESTGIVER_QUEST_AUTOLAUNCH {
}

impl crate::Message for CMSG_QUESTGIVER_QUEST_AUTOLAUNCH {
    const OPCODE: u32 = 0x0187;

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
#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ClientMessage for CMSG_QUESTGIVER_QUEST_AUTOLAUNCH {}

