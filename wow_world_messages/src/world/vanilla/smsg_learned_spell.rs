use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_learned_spell.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_learned_spell.wowm#L3):
/// ```text
/// smsg SMSG_LEARNED_SPELL = 0x012B {
///     u32 id;
/// }
/// ```
pub struct SMSG_LEARNED_SPELL {
    pub id: u32,
}

impl crate::Message for SMSG_LEARNED_SPELL {
    const OPCODE: u32 = 0x012b;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        Ok(Self {
            id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_LEARNED_SPELL {}

