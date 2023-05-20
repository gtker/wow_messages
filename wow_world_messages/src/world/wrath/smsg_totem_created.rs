use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_totem_created.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_totem_created.wowm#L10):
/// ```text
/// smsg SMSG_TOTEM_CREATED = 0x0413 {
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

impl crate::private::Sealed for SMSG_TOTEM_CREATED {}
impl crate::Message for SMSG_TOTEM_CREATED {
    const OPCODE: u32 = 0x0413;

    fn size_without_header(&self) -> u32 {
        17
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 17 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0413, size: body_size });
        }

        // slot: u8
        let slot = crate::util::read_u8_le(&mut r)?;

        // totem: Guid
        let totem = Guid::read(&mut r)?;

        // duration: u32
        let duration = crate::util::read_u32_le(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            slot,
            totem,
            duration,
            spell,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_TOTEM_CREATED {}

