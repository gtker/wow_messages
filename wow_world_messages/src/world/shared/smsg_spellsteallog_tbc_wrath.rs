use std::io::{Read, Write};

use crate::Guid;
use crate::shared::spell_steal_tbc_wrath::SpellSteal;
use wow_world_base::shared::spell_steal_action_tbc_wrath::SpellStealAction;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spellsteallog.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spellsteallog.wowm#L15):
/// ```text
/// smsg SMSG_SPELLSTEALLOG = 0x0333 {
///     PackedGuid victim;
///     PackedGuid caster;
///     u32 spell;
///     u8 unknown;
///     u32 amount_of_spell_steals;
///     SpellSteal[amount_of_spell_steals] spell_steals;
/// }
/// ```
pub struct SMSG_SPELLSTEALLOG {
    pub victim: Guid,
    pub caster: Guid,
    pub spell: u32,
    pub unknown: u8,
    pub spell_steals: Vec<SpellSteal>,
}

impl crate::private::Sealed for SMSG_SPELLSTEALLOG {}
impl SMSG_SPELLSTEALLOG {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(13..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x0333, size: body_size });
        }

        // victim: PackedGuid
        let victim = crate::util::read_packed_guid(&mut r)?;

        // caster: PackedGuid
        let caster = crate::util::read_packed_guid(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // unknown: u8
        let unknown = crate::util::read_u8_le(&mut r)?;

        // amount_of_spell_steals: u32
        let amount_of_spell_steals = crate::util::read_u32_le(&mut r)?;

        // spell_steals: SpellSteal[amount_of_spell_steals]
        let spell_steals = {
            let mut spell_steals = Vec::with_capacity(amount_of_spell_steals as usize);
            for _ in 0..amount_of_spell_steals {
                spell_steals.push(SpellSteal::read(&mut r)?);
            }
            spell_steals
        };

        Ok(Self {
            victim,
            caster,
            spell,
            unknown,
            spell_steals,
        })
    }

}

impl crate::Message for SMSG_SPELLSTEALLOG {
    const OPCODE: u32 = 0x0333;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SPELLSTEALLOG {{").unwrap();
        // Members
        writeln!(s, "    victim = {};", self.victim.guid()).unwrap();
        writeln!(s, "    caster = {};", self.caster.guid()).unwrap();
        writeln!(s, "    spell = {};", self.spell).unwrap();
        writeln!(s, "    unknown = {};", self.unknown).unwrap();
        writeln!(s, "    amount_of_spell_steals = {};", self.spell_steals.len()).unwrap();
        write!(s, "    spell_steals = [").unwrap();
        for v in self.spell_steals.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        spell = {};", v.spell).unwrap();
            writeln!(s, "        action = {};", v.action.as_test_case_value()).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 819_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.victim), "victim", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.caster), "caster", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_spell_steals", "    ");
        if !self.spell_steals.is_empty() {
            writeln!(s, "    /* spell_steals: SpellSteal[amount_of_spell_steals] start */").unwrap();
            for (i, v) in self.spell_steals.iter().enumerate() {
                writeln!(s, "    /* spell_steals: SpellSteal[amount_of_spell_steals] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "action", "        ");
                writeln!(s, "    /* spell_steals: SpellSteal[amount_of_spell_steals] {i} end */").unwrap();
            }
            writeln!(s, "    /* spell_steals: SpellSteal[amount_of_spell_steals] end */").unwrap();
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

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // unknown: u8
        w.write_all(&self.unknown.to_le_bytes())?;

        // amount_of_spell_steals: u32
        w.write_all(&(self.spell_steals.len() as u32).to_le_bytes())?;

        // spell_steals: SpellSteal[amount_of_spell_steals]
        for i in self.spell_steals.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPELLSTEALLOG {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPELLSTEALLOG {}

impl SMSG_SPELLSTEALLOG {
    pub(crate) fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.victim) // victim: PackedGuid
        + crate::util::packed_guid_size(&self.caster) // caster: PackedGuid
        + 4 // spell: u32
        + 1 // unknown: u8
        + 4 // amount_of_spell_steals: u32
        + self.spell_steals.len() * 5 // spell_steals: SpellSteal[amount_of_spell_steals]
    }
}

