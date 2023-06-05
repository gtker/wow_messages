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

#[cfg(feature = "print-testcase")]
impl SMSG_AURA_UPDATE {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_AURA_UPDATE {{").unwrap();
        // Members
        writeln!(s, "    unit = {};", self.unit.guid()).unwrap();
        // aura_update: AuraUpdate
        writeln!(s, "    aura_update = {{").unwrap();
        // Members
        writeln!(s, "    visual_slot = {};", self.aura_update.visual_slot).unwrap();
        writeln!(s, "    spell = {};", self.aura_update.spell).unwrap();
        writeln!(s, "    flags = {};", crate::wrath::AuraFlag::new(self.aura_update.flags.as_int()).as_test_case_value()).unwrap();
        writeln!(s, "    level = {};", self.aura_update.level.as_int()).unwrap();
        writeln!(s, "    aura_stack_count = {};", self.aura_update.aura_stack_count).unwrap();
        if let Some(if_statement) = &self.aura_update.flags.get_not_caster() {
            writeln!(s, "    caster = {};", if_statement.caster.guid()).unwrap();
        }

        if let Some(if_statement) = &self.aura_update.flags.get_duration() {
            writeln!(s, "    duration = {};", if_statement.duration).unwrap();
            writeln!(s, "    time_left = {};", if_statement.time_left).unwrap();
        }


        writeln!(s, "    }};").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1174_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_AURA_UPDATE {}
impl crate::Message for SMSG_AURA_UPDATE {
    const OPCODE: u32 = 0x0496;

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

