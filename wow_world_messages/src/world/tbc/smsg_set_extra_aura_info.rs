use std::io::{Read, Write};
use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_set_extra_aura_info.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_set_extra_aura_info.wowm#L1):
/// ```text
/// smsg SMSG_SET_EXTRA_AURA_INFO = 0x03A4 {
///     PackedGuid unit;
///     optional aura {
///         u8 slot;
///         u32 spell;
///         u32 max_duration;
///         u32 remaining_duration;
///     }
/// }
/// ```
pub struct SMSG_SET_EXTRA_AURA_INFO {
    pub unit: Guid,
    pub aura: Option<SMSG_SET_EXTRA_AURA_INFO_aura>,
}

impl crate::Message for SMSG_SET_EXTRA_AURA_INFO {
    const OPCODE: u32 = 0x03a4;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        self.unit.write_packed_guid_into_vec(&mut w)?;

        // optional aura
        if let Some(v) = &self.aura {
            // slot: u8
            w.write_all(&v.slot.to_le_bytes())?;

            // spell: u32
            w.write_all(&v.spell.to_le_bytes())?;

            // max_duration: u32
            w.write_all(&v.max_duration.to_le_bytes())?;

            // remaining_duration: u32
            w.write_all(&v.remaining_duration.to_le_bytes())?;

        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=22).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03A4, size: body_size as u32 });
        }

        // unit: PackedGuid
        let unit = Guid::read_packed(&mut r)?;

        // optional aura
        let current_size = {
            unit.size() // unit: PackedGuid
        };
        let aura = if current_size < body_size as usize {
            // slot: u8
            let slot = crate::util::read_u8_le(&mut r)?;

            // spell: u32
            let spell = crate::util::read_u32_le(&mut r)?;

            // max_duration: u32
            let max_duration = crate::util::read_u32_le(&mut r)?;

            // remaining_duration: u32
            let remaining_duration = crate::util::read_u32_le(&mut r)?;

            Some(SMSG_SET_EXTRA_AURA_INFO_aura {
                slot,
                spell,
                max_duration,
                remaining_duration,
            })
        } else {
            None
        };

        Ok(Self {
            unit,
            aura,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SET_EXTRA_AURA_INFO {}

impl SMSG_SET_EXTRA_AURA_INFO {
    pub(crate) fn size(&self) -> usize {
        self.unit.size() // unit: PackedGuid
        + if let Some(aura) = &self.aura {
            1 // slot: u8
            + 4 // spell: u32
            + 4 // max_duration: u32
            + 4 // remaining_duration: u32
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_SET_EXTRA_AURA_INFO_aura {
    pub slot: u8,
    pub spell: u32,
    pub max_duration: u32,
    pub remaining_duration: u32,
}

impl SMSG_SET_EXTRA_AURA_INFO_aura {
    pub(crate) fn size(&self) -> usize {
        1 // slot: u8
        + 4 // spell: u32
        + 4 // max_duration: u32
        + 4 // remaining_duration: u32
    }

}

