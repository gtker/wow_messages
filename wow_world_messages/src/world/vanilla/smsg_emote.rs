use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::Emote;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_emote.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_emote.wowm#L3):
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

impl crate::Message for SMSG_EMOTE {
    const OPCODE: u32 = 0x0103;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // emote: Emote
        w.write_all(&(self.emote.as_int() as u32).to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // emote: Emote
        let emote: Emote = crate::util::read_u32_le(r)?.try_into()?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            emote,
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_EMOTE {}

