use std::io::{Read, Write};

use crate::Guid;
use crate::shared::dispelled_spell_tbc_wrath::DispelledSpell;
use wow_world_base::shared::dispel_method_tbc_wrath::DispelMethod;

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
    pub unknown: u8,
    pub spells: Vec<DispelledSpell>,
}

impl crate::private::Sealed for SMSG_SPELLDISPELLOG {}
impl SMSG_SPELLDISPELLOG {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(11..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
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

impl crate::Message for SMSG_SPELLDISPELLOG {
    const OPCODE: u32 = 0x027b;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_SPELLDISPELLOG"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
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
        writeln!(s, "    spells = [").unwrap();
        for v in self.spells.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            spell = {};", v.spell).unwrap();
            writeln!(s, "            method = {};", v.method.as_test_case_value()).unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 635_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.victim), "victim", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.caster), "caster", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "dispell_spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_spells", "    ");
        if !self.spells.is_empty() {
            writeln!(s, "    /* spells: DispelledSpell[amount_of_spells] start */").unwrap();
            for (i, v) in self.spells.iter().enumerate() {
                writeln!(s, "    /* spells: DispelledSpell[amount_of_spells] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "method", "        ");
                writeln!(s, "    /* spells: DispelledSpell[amount_of_spells] {i} end */").unwrap();
            }
            writeln!(s, "    /* spells: DispelledSpell[amount_of_spells] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(635, "SMSG_SPELLDISPELLOG", body_size, a))
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

