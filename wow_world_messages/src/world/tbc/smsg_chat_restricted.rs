use crate::tbc::ChatRestrictionType;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_chat_restricted.wowm:22`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_chat_restricted.wowm#L22):
/// ```text
/// smsg SMSG_CHAT_RESTRICTED = 0x02FD {
///     ChatRestrictionType restriction;
/// }
/// ```
pub struct SMSG_CHAT_RESTRICTED {
    pub restriction: ChatRestrictionType,
}

impl crate::Message for SMSG_CHAT_RESTRICTED {
    const OPCODE: u32 = 0x02fd;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // restriction: ChatRestrictionType
        w.write_all(&u8::from(self.restriction.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02FD, size: body_size as u32 });
        }

        // restriction: ChatRestrictionType
        let restriction: ChatRestrictionType = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            restriction,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_CHAT_RESTRICTED {}

