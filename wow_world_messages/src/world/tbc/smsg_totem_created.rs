use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_totem_created.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_totem_created.wowm#L1):
/// ```text
/// smsg SMSG_TOTEM_CREATED = 0x0412 {
///     u8 slot;
///     Guid totem;
///     u32 duration;
///     u32 spell;
/// }
/// ```
pub struct SMSG_TOTEM_CREATED {
    pub slot: u8,
    pub totem: Guid,
    pub duration: u32,
    pub spell: u32,
}

impl crate::Message for SMSG_TOTEM_CREATED {
    const OPCODE: u32 = 0x0412;

    fn size_without_header(&self) -> u32 {
        17
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        // totem: Guid
        w.write_all(&self.totem.guid().to_le_bytes())?;

        // duration: u32
        w.write_all(&self.duration.to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 17 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0412, size: body_size as u32 });
        }

        // slot: u8
        let slot = crate::util::read_u8_le(r)?;

        // totem: Guid
        let totem = Guid::read(r)?;

        // duration: u32
        let duration = crate::util::read_u32_le(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        Ok(Self {
            slot,
            totem,
            duration,
            spell,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_TOTEM_CREATED {}

