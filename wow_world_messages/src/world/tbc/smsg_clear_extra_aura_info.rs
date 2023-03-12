use std::io::{Read, Write};
use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_clear_extra_aura_info.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_clear_extra_aura_info.wowm#L1):
/// ```text
/// smsg SMSG_CLEAR_EXTRA_AURA_INFO = 0x03A6 {
///     PackedGuid unit;
///     u32 spell;
/// }
/// ```
pub struct SMSG_CLEAR_EXTRA_AURA_INFO {
    pub unit: Guid,
    pub spell: u32,
}

impl crate::Message for SMSG_CLEAR_EXTRA_AURA_INFO {
    const OPCODE: u32 = 0x03a6;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        self.unit.write_packed_guid_into_vec(&mut w)?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(6..=13).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03A6, size: body_size as u32 });
        }

        // unit: PackedGuid
        let unit = Guid::read_packed(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            unit,
            spell,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_CLEAR_EXTRA_AURA_INFO {}

impl SMSG_CLEAR_EXTRA_AURA_INFO {
    pub(crate) fn size(&self) -> usize {
        self.unit.size() // unit: PackedGuid
        + 4 // spell: u32
    }
}

