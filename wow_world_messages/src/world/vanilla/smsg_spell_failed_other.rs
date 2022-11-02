use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spell_failed_other.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spell_failed_other.wowm#L3):
/// ```text
/// smsg SMSG_SPELL_FAILED_OTHER = 0x02A6 {
///     Guid caster_guid;
///     u32 id;
/// }
/// ```
pub struct SMSG_SPELL_FAILED_OTHER {
    pub caster_guid: Guid,
    pub id: u32,
}

impl crate::Message for SMSG_SPELL_FAILED_OTHER {
    const OPCODE: u32 = 0x02a6;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // caster_guid: Guid
        w.write_all(&self.caster_guid.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02A6, size: body_size as u32 });
        }

        // caster_guid: Guid
        let caster_guid = Guid::read(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        Ok(Self {
            caster_guid,
            id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_SPELL_FAILED_OTHER {}

