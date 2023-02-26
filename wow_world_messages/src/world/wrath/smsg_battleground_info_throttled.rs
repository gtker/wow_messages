use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Only exists as a comment in azerothcore.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battleground_info_throttled.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battleground_info_throttled.wowm#L1):
/// ```text
/// smsg SMSG_BATTLEGROUND_INFO_THROTTLED = 0x04A6 {
/// }
/// ```
pub struct SMSG_BATTLEGROUND_INFO_THROTTLED {
}

impl crate::Message for SMSG_BATTLEGROUND_INFO_THROTTLED {
    const OPCODE: u32 = 0x04a6;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04A6, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BATTLEGROUND_INFO_THROTTLED {}

