use std::io::{Read, Write};
use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_set_extra_aura_info_need_update.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_set_extra_aura_info_need_update.wowm#L1):
/// ```text
/// smsg SMSG_SET_EXTRA_AURA_INFO_NEED_UPDATE = 0x03A5 {
///     PackedGuid unit;
///     u8 slot;
///     u32 spell;
///     u32 max_duration;
///     u32 remaining_duration;
/// }
/// ```
pub struct SMSG_SET_EXTRA_AURA_INFO_NEED_UPDATE {
    pub unit: Guid,
    pub slot: u8,
    pub spell: u32,
    pub max_duration: u32,
    pub remaining_duration: u32,
}

impl crate::Message for SMSG_SET_EXTRA_AURA_INFO_NEED_UPDATE {
    const OPCODE: u32 = 0x03a5;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        self.unit.write_packed_guid_into_vec(&mut w)?;

        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // max_duration: u32
        w.write_all(&self.max_duration.to_le_bytes())?;

        // remaining_duration: u32
        w.write_all(&self.remaining_duration.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(15..=22).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03A5, size: body_size as u32 });
        }

        // unit: PackedGuid
        let unit = Guid::read_packed(&mut r)?;

        // slot: u8
        let slot = crate::util::read_u8_le(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // max_duration: u32
        let max_duration = crate::util::read_u32_le(&mut r)?;

        // remaining_duration: u32
        let remaining_duration = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            unit,
            slot,
            spell,
            max_duration,
            remaining_duration,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SET_EXTRA_AURA_INFO_NEED_UPDATE {}

impl SMSG_SET_EXTRA_AURA_INFO_NEED_UPDATE {
    pub(crate) fn size(&self) -> usize {
        self.unit.size() // unit: PackedGuid
        + 1 // slot: u8
        + 4 // spell: u32
        + 4 // max_duration: u32
        + 4 // remaining_duration: u32
    }
}

