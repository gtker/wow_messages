use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::SpellSchool;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelldamageshield.wowm:20`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelldamageshield.wowm#L20):
/// ```text
/// smsg SMSG_SPELLDAMAGESHIELD = 0x024F {
///     Guid victim;
///     Guid caster;
///     u32 spell;
///     u32 damage;
///     u32 overkill;
///     (u32)SpellSchool school;
/// }
/// ```
pub struct SMSG_SPELLDAMAGESHIELD {
    pub victim: Guid,
    pub caster: Guid,
    pub spell: u32,
    pub damage: u32,
    pub overkill: u32,
    pub school: SpellSchool,
}

#[cfg(feature = "print-testcase")]
impl SMSG_SPELLDAMAGESHIELD {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SPELLDAMAGESHIELD {{").unwrap();
        // Members
        writeln!(s, "    victim = {};", self.victim.guid()).unwrap();
        writeln!(s, "    caster = {};", self.caster.guid()).unwrap();
        writeln!(s, "    spell = {};", self.spell).unwrap();
        writeln!(s, "    damage = {};", self.damage).unwrap();
        writeln!(s, "    overkill = {};", self.overkill).unwrap();
        writeln!(s, "    school = {};", self.school.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 36_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 591_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "victim");
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

impl crate::private::Sealed for SMSG_SPELLDAMAGESHIELD {}
impl crate::Message for SMSG_SPELLDAMAGESHIELD {
    const OPCODE: u32 = 0x024f;

    fn size_without_header(&self) -> u32 {
        32
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

        // overkill: u32
        w.write_all(&self.overkill.to_le_bytes())?;

        // school: SpellSchool
        w.write_all(&u32::from(self.school.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 32 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x024F, size: body_size });
        }

        // victim: Guid
        let victim = crate::util::read_guid(&mut r)?;

        // caster: Guid
        let caster = crate::util::read_guid(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // damage: u32
        let damage = crate::util::read_u32_le(&mut r)?;

        // overkill: u32
        let overkill = crate::util::read_u32_le(&mut r)?;

        // school: SpellSchool
        let school = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        Ok(Self {
            victim,
            caster,
            spell,
            damage,
            overkill,
            school,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPELLDAMAGESHIELD {}

