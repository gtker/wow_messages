use std::io::{Read, Write};

use crate::Guid;
use crate::shared::dispelled_spell_tbc_wrath::DispelledSpell;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelldispellog.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelldispellog.wowm#L10):
/// ```text
/// smsg SMSG_SPELLDISPELLOG = 0x027B {
///     PackedGuid victim;
///     PackedGuid caster;
///     u32 dispell_spell;
///     u8 unknown;
///     u32 amount_of_spells;
///     DispelledSpell[amount_of_spells] spells;
/// }
/// ```
pub struct SMSG_SPELLDISPELLOG {
    pub victim: Guid,
    pub caster: Guid,
    pub dispell_spell: u32,
    /// mangosone: unused
    ///
    pub unknown: u8,
    pub spells: Vec<DispelledSpell>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_SPELLDISPELLOG {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SPELLDISPELLOG {{").unwrap();
        // Members
        writeln!(s, "    victim = {};", self.victim.guid()).unwrap();
        writeln!(s, "    caster = {};", self.caster.guid()).unwrap();
        writeln!(s, "    dispell_spell = {};", self.dispell_spell).unwrap();
        writeln!(s, "    unknown = {};", self.unknown).unwrap();
        writeln!(s, "    amount_of_spells = {};", self.spells.len()).unwrap();
        write!(s, "    spells = [").unwrap();
        for v in self.spells.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "    spell = {};", v.spell).unwrap();
            writeln!(s, "    method = {};", v.method.as_test_case_value()).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 635_u32.to_le_bytes();
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
        writeln!(s, "    versions = \"2.4.3 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_SPELLDISPELLOG {}
impl crate::Message for SMSG_SPELLDISPELLOG {
    const OPCODE: u32 = 0x027b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // victim: PackedGuid
        crate::util::write_packed_guid(&self.victim, &mut w)?;

        // caster: PackedGuid
        crate::util::write_packed_guid(&self.caster, &mut w)?;

        // dispell_spell: u32
        w.write_all(&self.dispell_spell.to_le_bytes())?;

        // unknown: u8
        w.write_all(&self.unknown.to_le_bytes())?;

        // amount_of_spells: u32
        w.write_all(&(self.spells.len() as u32).to_le_bytes())?;

        // spells: DispelledSpell[amount_of_spells]
        for i in self.spells.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(13..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x027B, size: body_size });
        }

        // victim: PackedGuid
        let victim = crate::util::read_packed_guid(&mut r)?;

        // caster: PackedGuid
        let caster = crate::util::read_packed_guid(&mut r)?;

        // dispell_spell: u32
        let dispell_spell = crate::util::read_u32_le(&mut r)?;

        // unknown: u8
        let unknown = crate::util::read_u8_le(&mut r)?;

        // amount_of_spells: u32
        let amount_of_spells = crate::util::read_u32_le(&mut r)?;

        // spells: DispelledSpell[amount_of_spells]
        let spells = {
            let mut spells = Vec::with_capacity(amount_of_spells as usize);
            for _ in 0..amount_of_spells {
                spells.push(DispelledSpell::read(&mut r)?);
            }
            spells
        };

        Ok(Self {
            victim,
            caster,
            dispell_spell,
            unknown,
            spells,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPELLDISPELLOG {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPELLDISPELLOG {}

impl SMSG_SPELLDISPELLOG {
    pub(crate) fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.victim) // victim: PackedGuid
        + crate::util::packed_guid_size(&self.caster) // caster: PackedGuid
        + 4 // dispell_spell: u32
        + 1 // unknown: u8
        + 4 // amount_of_spells: u32
        + self.spells.len() * 5 // spells: DispelledSpell[amount_of_spells]
    }
}

