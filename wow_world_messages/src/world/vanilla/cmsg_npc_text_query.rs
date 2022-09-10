use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/cmsg_npc_text_query.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_npc_text_query.wowm#L3):
/// ```text
/// cmsg CMSG_NPC_TEXT_QUERY = 0x017F {
///     u32 text_id;
///     Guid guid;
/// }
/// ```
pub struct CMSG_NPC_TEXT_QUERY {
    pub text_id: u32,
    pub guid: Guid,
}

impl crate::Message for CMSG_NPC_TEXT_QUERY {
    const OPCODE: u32 = 0x017f;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // text_id: u32
        w.write_all(&self.text_id.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // text_id: u32
        let text_id = crate::util::read_u32_le(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            text_id,
            guid,
        })
    }

}
#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ClientMessage for CMSG_NPC_TEXT_QUERY {}

