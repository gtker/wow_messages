use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::SpellSchool;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelldamageshield.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelldamageshield.wowm#L10):
/// ```text
/// smsg SMSG_SPELLDAMAGESHIELD = 0x024F {
///     Guid victim;
///     Guid caster;
///     u32 spell;
///     u32 damage;
///     (u32)SpellSchool school;
/// }
/// ```
pub struct SMSG_SPELLDAMAGESHIELD {
    pub victim: Guid,
    pub caster: Guid,
    pub spell: u32,
    pub damage: u32,
    pub school: SpellSchool,
}

impl crate::private::Sealed for SMSG_SPELLDAMAGESHIELD {}
impl SMSG_SPELLDAMAGESHIELD {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 28 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // victim: Guid
        let victim = crate::util::read_guid(&mut r)?;

        // caster: Guid
        let caster = crate::util::read_guid(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // damage: u32
        let damage = crate::util::read_u32_le(&mut r)?;

        // school: SpellSchool
        let school = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        Ok(Self {
            victim,
            caster,
            spell,
            damage,
            school,
        })
    }

}

impl crate::Message for SMSG_SPELLDAMAGESHIELD {
    const OPCODE: u32 = 0x024f;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SPELLDAMAGESHIELD {{").unwrap();
        // Members
        writeln!(s, "    victim = {};", self.victim.guid()).unwrap();
        writeln!(s, "    caster = {};", self.caster.guid()).unwrap();
        writeln!(s, "    spell = {};", self.spell).unwrap();
        writeln!(s, "    damage = {};", self.damage).unwrap();
        writeln!(s, "    school = {};", self.school.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 30_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 591_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "victim", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "caster", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "damage", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "school", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        28
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // victim: Guid
        w.write_all(&self.victim.guid().to_le_bytes())?;

        // caster: Guid
        w.write_all(&self.caster.guid().to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // school: SpellSchool
        w.write_all(&u32::from(self.school.as_int()).to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(591, "SMSG_SPELLDAMAGESHIELD", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPELLDAMAGESHIELD {}

