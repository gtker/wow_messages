use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::Emote;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_emote.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_emote.wowm#L1):
/// ```text
/// smsg SMSG_EMOTE = 0x0103 {
///     Emote emote;
///     Guid guid;
/// }
/// ```
pub struct SMSG_EMOTE {
    pub emote: Emote,
    pub guid: Guid,
}

impl crate::private::Sealed for SMSG_EMOTE {}
impl crate::Message for SMSG_EMOTE {
    const OPCODE: u32 = 0x0103;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // emote: Emote
        w.write_all(&(self.emote.as_int().to_le_bytes()))?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0103, size: body_size });
        }

        // emote: Emote
        let emote: Emote = crate::util::read_u32_le(&mut r)?.try_into()?;

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        Ok(Self {
            emote,
            guid,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_EMOTE {}

