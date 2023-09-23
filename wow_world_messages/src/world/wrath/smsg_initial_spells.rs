use std::io::{Read, Write};

use crate::wrath::{
    CooldownSpell, InitialSpell,
};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_initial_spells.wowm:43`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_initial_spells.wowm#L43):
/// ```text
/// smsg SMSG_INITIAL_SPELLS = 0x012A {
///     u8 unknown1;
///     u16 spell_count;
///     InitialSpell[spell_count] initial_spells;
///     u16 cooldown_count;
///     CooldownSpell[cooldown_count] cooldowns;
/// }
/// ```
pub struct SMSG_INITIAL_SPELLS {
    /// cmangos/mangoszero: sets to 0
    pub unknown1: u8,
    pub initial_spells: Vec<InitialSpell>,
    pub cooldowns: Vec<CooldownSpell>,
}

impl crate::private::Sealed for SMSG_INITIAL_SPELLS {}
impl SMSG_INITIAL_SPELLS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(5..=1310725).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(&mut r)?;

        // spell_count: u16
        let spell_count = crate::util::read_u16_le(&mut r)?;

        // initial_spells: InitialSpell[spell_count]
        let initial_spells = {
            let mut initial_spells = Vec::with_capacity(spell_count as usize);
            for _ in 0..spell_count {
                initial_spells.push(InitialSpell::read(&mut r)?);
            }
            initial_spells
        };

        // cooldown_count: u16
        let cooldown_count = crate::util::read_u16_le(&mut r)?;

        // cooldowns: CooldownSpell[cooldown_count]
        let cooldowns = {
            let mut cooldowns = Vec::with_capacity(cooldown_count as usize);
            for _ in 0..cooldown_count {
                cooldowns.push(CooldownSpell::read(&mut r)?);
            }
            cooldowns
        };

        Ok(Self {
            unknown1,
            initial_spells,
            cooldowns,
        })
    }

}

impl crate::Message for SMSG_INITIAL_SPELLS {
    const OPCODE: u32 = 0x012a;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_INITIAL_SPELLS"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_INITIAL_SPELLS {{").unwrap();
        // Members
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        writeln!(s, "    spell_count = {};", self.initial_spells.len()).unwrap();
        writeln!(s, "    initial_spells = [").unwrap();
        for v in self.initial_spells.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            spell_id = {};", v.spell_id).unwrap();
            writeln!(s, "            unknown1 = {};", v.unknown1).unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();
        writeln!(s, "    cooldown_count = {};", self.cooldowns.len()).unwrap();
        writeln!(s, "    cooldowns = [").unwrap();
        for v in self.cooldowns.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            spell_id = {};", v.spell_id).unwrap();
            writeln!(s, "            item_id = {};", v.item_id).unwrap();
            writeln!(s, "            spell_category = {};", v.spell_category).unwrap();
            writeln!(s, "            cooldown = {};", v.cooldown.as_millis()).unwrap();
            writeln!(s, "            category_cooldown = {};", v.category_cooldown.as_millis()).unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 298_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 2, "spell_count", "    ");
        if !self.initial_spells.is_empty() {
            writeln!(s, "    /* initial_spells: InitialSpell[spell_count] start */").unwrap();
            for (i, v) in self.initial_spells.iter().enumerate() {
                writeln!(s, "    /* initial_spells: InitialSpell[spell_count] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "spell_id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 2, "unknown1", "        ");
                writeln!(s, "    /* initial_spells: InitialSpell[spell_count] {i} end */").unwrap();
            }
            writeln!(s, "    /* initial_spells: InitialSpell[spell_count] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 2, "cooldown_count", "    ");
        if !self.cooldowns.is_empty() {
            writeln!(s, "    /* cooldowns: CooldownSpell[cooldown_count] start */").unwrap();
            for (i, v) in self.cooldowns.iter().enumerate() {
                writeln!(s, "    /* cooldowns: CooldownSpell[cooldown_count] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 2, "spell_id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 2, "item_id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 2, "spell_category", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "cooldown", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "category_cooldown", "        ");
                writeln!(s, "    /* cooldowns: CooldownSpell[cooldown_count] {i} end */").unwrap();
            }
            writeln!(s, "    /* cooldowns: CooldownSpell[cooldown_count] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // spell_count: u16
        w.write_all(&(self.initial_spells.len() as u16).to_le_bytes())?;

        // initial_spells: InitialSpell[spell_count]
        for i in self.initial_spells.iter() {
            i.write_into_vec(&mut w)?;
        }

        // cooldown_count: u16
        w.write_all(&(self.cooldowns.len() as u16).to_le_bytes())?;

        // cooldowns: CooldownSpell[cooldown_count]
        for i in self.cooldowns.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(298, "SMSG_INITIAL_SPELLS", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_INITIAL_SPELLS {}

impl SMSG_INITIAL_SPELLS {
    pub(crate) fn size(&self) -> usize {
        1 // unknown1: u8
        + 2 // spell_count: u16
        + self.initial_spells.len() * 6 // initial_spells: InitialSpell[spell_count]
        + 2 // cooldown_count: u16
        + self.cooldowns.len() * 14 // cooldowns: CooldownSpell[cooldown_count]
    }
}

