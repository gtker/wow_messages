use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::AuraUpdate;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_aura_update.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_aura_update.wowm#L1):
/// ```text
/// smsg SMSG_AURA_UPDATE = 0x0496 {
///     PackedGuid unit;
///     AuraUpdate aura_update;
/// }
/// ```
pub struct SMSG_AURA_UPDATE {
    pub unit: Guid,
    pub aura_update: AuraUpdate,
}

impl crate::private::Sealed for SMSG_AURA_UPDATE {}
impl crate::Message for SMSG_AURA_UPDATE {
    const OPCODE: u32 = 0x0496;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_AURA_UPDATE {{").unwrap();
        // Members
        writeln!(s, "    unit = {};", self.unit.guid()).unwrap();
        // aura_update: AuraUpdate
        writeln!(s, "    aura_update = {{").unwrap();
        // Members
        writeln!(s, "        visual_slot = {};", self.aura_update.visual_slot).unwrap();
        writeln!(s, "        spell = {};", self.aura_update.spell).unwrap();
        writeln!(s, "        flags = {};", crate::wrath::AuraFlag::new(self.aura_update.flags.as_int()).as_test_case_value()).unwrap();
        writeln!(s, "        level = {};", self.aura_update.level.as_int()).unwrap();
        writeln!(s, "        aura_stack_count = {};", self.aura_update.aura_stack_count).unwrap();
        if let Some(if_statement) = &self.aura_update.flags.get_not_caster() {
            writeln!(s, "        caster = {};", if_statement.caster.guid()).unwrap();
        }

        if let Some(if_statement) = &self.aura_update.flags.get_duration() {
            writeln!(s, "        duration = {};", if_statement.duration).unwrap();
            writeln!(s, "        time_left = {};", if_statement.time_left).unwrap();
        }


        writeln!(s, "    }};").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1174_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.unit), "unit", "    ");
        writeln!(s, "    /* aura_update: AuraUpdate start */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 1, "visual_slot", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "flags", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "level", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "aura_stack_count", "        ");
        if let Some(if_statement) = &self.aura_update.flags.get_not_caster() {
            crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&if_statement.caster), "caster", "        ");
        }

        if let Some(if_statement) = &self.aura_update.flags.get_duration() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, "duration", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "time_left", "        ");
        }

        writeln!(s, "    /* aura_update: AuraUpdate end */").unwrap();


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        crate::util::write_packed_guid(&self.unit, &mut w)?;

        // aura_update: AuraUpdate
        self.aura_update.write_into_vec(&mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(10..=34).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0496, size: body_size });
        }

        // unit: PackedGuid
        let unit = crate::util::read_packed_guid(&mut r)?;

        // aura_update: AuraUpdate
        let aura_update = AuraUpdate::read(&mut r)?;

        Ok(Self {
            unit,
            aura_update,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_AURA_UPDATE {}

impl SMSG_AURA_UPDATE {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.unit) // unit: PackedGuid
        + self.aura_update.size() // aura_update: AuraUpdate
    }
}

