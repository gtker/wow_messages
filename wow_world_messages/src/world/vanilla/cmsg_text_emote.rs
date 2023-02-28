use crate::Guid;
use crate::vanilla::Emote;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_text_emote.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_text_emote.wowm#L1):
/// ```text
/// cmsg CMSG_TEXT_EMOTE = 0x0104 {
///     u32 text_emote;
///     Emote emote;
///     Guid guid;
/// }
/// ```
pub struct CMSG_TEXT_EMOTE {
    pub text_emote: u32,
    pub emote: Emote,
    pub guid: Guid,
}

impl crate::Message for CMSG_TEXT_EMOTE {
    const OPCODE: u32 = 0x0104;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // text_emote: u32
        w.write_all(&self.text_emote.to_le_bytes())?;

        // emote: Emote
        w.write_all(&u32::from(self.emote.as_int()).to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0104, size: body_size as u32 });
        }

        // text_emote: u32
        let text_emote = crate::util::read_u32_le(r)?;

        // emote: Emote
        let emote: Emote = crate::util::read_u32_le(r)?.try_into()?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            text_emote,
            emote,
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_TEXT_EMOTE {}

