use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::Power;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spellenergizelog.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spellenergizelog.wowm#L11):
/// ```text
/// smsg SMSG_SPELLENERGIZELOG = 0x0151 {
///     PackedGuid victim;
///     PackedGuid caster;
///     u32 spell;
///     (u32)Power power;
///     u32 damage;
/// }
/// ```
pub struct SMSG_SPELLENERGIZELOG {
    pub victim: Guid,
    pub caster: Guid,
    pub spell: u32,
    pub power: Power,
    pub damage: u32,
}

impl crate::private::Sealed for SMSG_SPELLENERGIZELOG {}
impl SMSG_SPELLENERGIZELOG {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(16..=30).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // victim: PackedGuid
        let victim = crate::util::read_packed_guid(&mut r)?;

        // caster: PackedGuid
        let caster = crate::util::read_packed_guid(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // power: Power
        let power = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // damage: u32
        let damage = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            victim,
            caster,
            spell,
            power,
            damage,
        })
    }

}

impl crate::Message for SMSG_SPELLENERGIZELOG {
    const OPCODE: u32 = 0x0151;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SPELLENERGIZELOG {{").unwrap();
        // Members
        writeln!(s, "    victim = {};", self.victim.guid()).unwrap();
        writeln!(s, "    caster = {};", self.caster.guid()).unwrap();
        writeln!(s, "    spell = {};", self.spell).unwrap();
        writeln!(s, "    power = {};", self.power.as_test_case_value()).unwrap();
        writeln!(s, "    damage = {};", self.damage).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 337_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.victim), "victim", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.caster), "caster", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "power", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "damage", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3".to_string())).unwrap();
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

        // power: Power
        w.write_all(&u32::from(self.power.as_int()).to_le_bytes())?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(337, "SMSG_SPELLENERGIZELOG", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPELLENERGIZELOG {}

impl SMSG_SPELLENERGIZELOG {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.victim) // victim: PackedGuid
        + crate::util::packed_guid_size(&self.caster) // caster: PackedGuid
        + 4 // spell: u32
        + 4 // power: Power
        + 4 // damage: u32
    }
}

