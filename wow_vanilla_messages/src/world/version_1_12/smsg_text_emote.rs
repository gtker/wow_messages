use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::Emote;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_text_emote.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_text_emote.wowm#L3):
/// ```text
/// smsg SMSG_TEXT_EMOTE = 0x0105 {
///     Guid guid;
///     u32 text_emote;
///     Emote emote;
///     u32 name_length;
///     CString name;
/// }
/// ```
pub struct SMSG_TEXT_EMOTE {
    pub guid: Guid,
    pub text_emote: u32,
    pub emote: Emote,
    pub name_length: u32,
    pub name: String,
}

impl ServerMessage for SMSG_TEXT_EMOTE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // text_emote: u32
        w.write_all(&self.text_emote.to_le_bytes())?;

        // emote: Emote
        w.write_all(&(self.emote.as_int() as u32).to_le_bytes())?;

        // name_length: u32
        w.write_all(&self.name_length.to_le_bytes())?;

        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x0105;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // text_emote: u32
        let text_emote = crate::util::read_u32_le(r)?;

        // emote: Emote
        let emote: Emote = crate::util::read_u32_le(r)?.try_into()?;

        // name_length: u32
        let name_length = crate::util::read_u32_le(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        Ok(Self {
            guid,
            text_emote,
            emote,
            name_length,
            name,
        })
    }

}

impl SMSG_TEXT_EMOTE {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // text_emote: u32
        + 4 // emote: Emote
        + 4 // name_length: u32
        + self.name.len() + 1 // name: CString
    }
}

