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

#[cfg(feature = "print-testcase")]
impl SMSG_SET_EXTRA_AURA_INFO_NEED_UPDATE {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SET_EXTRA_AURA_INFO_NEED_UPDATE {{").unwrap();
        // Members
        writeln!(s, "    unit = {};", self.unit.guid()).unwrap();
        writeln!(s, "    slot = {};", self.slot).unwrap();
        writeln!(s, "    spell = {};", self.spell).unwrap();
        writeln!(s, "    max_duration = {};", self.max_duration).unwrap();
        writeln!(s, "    remaining_duration = {};", self.remaining_duration).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 933_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.unit), "unit", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "slot", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "max_duration", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "remaining_duration", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_SET_EXTRA_AURA_INFO_NEED_UPDATE {}
impl crate::Message for SMSG_SET_EXTRA_AURA_INFO_NEED_UPDATE {
    const OPCODE: u32 = 0x03a5;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_SET_EXTRA_AURA_INFO_NEED_UPDATE::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        crate::util::write_packed_guid(&self.unit, &mut w)?;

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

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(15..=22).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03A5, size: body_size });
        }

        // unit: PackedGuid
        let unit = crate::util::read_packed_guid(&mut r)?;

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
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.unit) // unit: PackedGuid
        + 1 // slot: u8
        + 4 // spell: u32
        + 4 // max_duration: u32
        + 4 // remaining_duration: u32
    }
}

