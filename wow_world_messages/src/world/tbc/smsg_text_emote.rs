use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::{
    Emote, TextEmote,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_text_emote.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_text_emote.wowm#L1):
/// ```text
/// smsg SMSG_TEXT_EMOTE = 0x0105 {
///     Guid guid;
///     TextEmote text_emote;
///     Emote emote;
///     SizedCString name;
/// }
/// ```
pub struct SMSG_TEXT_EMOTE {
    pub guid: Guid,
    pub text_emote: TextEmote,
    pub emote: Emote,
    pub name: String,
}

impl crate::private::Sealed for SMSG_TEXT_EMOTE {}
impl crate::Message for SMSG_TEXT_EMOTE {
    const OPCODE: u32 = 0x0105;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // text_emote: TextEmote
        w.write_all(&u32::from(self.text_emote.as_int()).to_le_bytes())?;

        // emote: Emote
        w.write_all(&u32::from(self.emote.as_int()).to_le_bytes())?;

        // name: SizedCString
        w.write_all(&((self.name.len() + 1) as u32).to_le_bytes())?;
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(21..=8020).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0105, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // text_emote: TextEmote
        let text_emote: TextEmote = crate::util::read_u32_le(&mut r)?.try_into()?;

        // emote: Emote
        let emote: Emote = crate::util::read_u32_le(&mut r)?.try_into()?;

        // name: SizedCString
        let name = {
            let name = crate::util::read_u32_le(&mut r)?;
            let name = crate::util::read_sized_c_string_to_vec(&mut r, name)?;
            String::from_utf8(name)?
        };

        Ok(Self {
            guid,
            text_emote,
            emote,
            name,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_TEXT_EMOTE {}

impl SMSG_TEXT_EMOTE {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // text_emote: TextEmote
        + 4 // emote: Emote
        + self.name.len() + 5 // name: SizedCString
    }
}

