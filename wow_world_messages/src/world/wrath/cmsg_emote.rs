use std::io::{Read, Write};

use crate::wrath::Emote;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_emote.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_emote.wowm#L1):
/// ```text
/// cmsg CMSG_EMOTE = 0x0102 {
///     Emote emote;
/// }
/// ```
pub struct CMSG_EMOTE {
    pub emote: Emote,
}

impl crate::private::Sealed for CMSG_EMOTE {}
impl crate::Message for CMSG_EMOTE {
    const OPCODE: u32 = 0x0102;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // emote: Emote
        w.write_all(&u32::from(self.emote.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0102, size: body_size as u32 });
        }

        // emote: Emote
        let emote: Emote = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            emote,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_EMOTE {}

