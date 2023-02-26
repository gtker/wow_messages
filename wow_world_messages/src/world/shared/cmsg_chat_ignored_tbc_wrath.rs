use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_chat_ignored.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_chat_ignored.wowm#L7):
/// ```text
/// cmsg CMSG_CHAT_IGNORED = 0x0225 {
///     Guid guid;
///     u8 unknown;
/// }
/// ```
pub struct CMSG_CHAT_IGNORED {
    pub guid: Guid,
    /// mangosone/arcemu/trinitycore/azerothcore: probably related to spam reporting
    ///
    pub unknown: u8,
}

impl crate::Message for CMSG_CHAT_IGNORED {
    const OPCODE: u32 = 0x0225;

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // unknown: u8
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0225, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // unknown: u8
        let unknown = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            unknown,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_CHAT_IGNORED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CHAT_IGNORED {}

