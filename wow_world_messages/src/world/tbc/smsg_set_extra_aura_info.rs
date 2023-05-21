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

impl crate::private::Sealed for SMSG_SET_EXTRA_AURA_INFO {}
impl SMSG_SET_EXTRA_AURA_INFO {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(2..=22).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // unit: PackedGuid
        let unit = crate::util::read_packed_guid(&mut r)?;

        // optional aura
        let current_size = {
            crate::util::packed_guid_size(&unit) // unit: PackedGuid
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

impl crate::Message for SMSG_SET_EXTRA_AURA_INFO {
    const OPCODE: u32 = 0x03a4;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SET_EXTRA_AURA_INFO {{").unwrap();
        // Members
        writeln!(s, "    unit = {};", self.unit.guid()).unwrap();
        if let Some(aura) = &self.aura {
            writeln!(s, "    slot = {};", aura.slot).unwrap();
            writeln!(s, "    spell = {};", aura.spell).unwrap();
            writeln!(s, "    max_duration = {};", aura.max_duration).unwrap();
            writeln!(s, "    remaining_duration = {};", aura.remaining_duration).unwrap();
        }

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 932_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.unit), "unit", "    ");
        if let Some(aura) = &self.aura {
            crate::util::write_bytes(&mut s, &mut bytes, 1, "slot", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "max_duration", "    ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "remaining_duration", "    ");
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        crate::util::write_packed_guid(&self.unit, &mut w)?;

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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(932, "SMSG_SET_EXTRA_AURA_INFO", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SET_EXTRA_AURA_INFO {}

impl SMSG_SET_EXTRA_AURA_INFO {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.unit) // unit: PackedGuid
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

