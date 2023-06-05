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

#[cfg(feature = "print-testcase")]
impl SMSG_SPELLENERGIZELOG {
    pub fn to_test_case_string(&self) -> String {
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

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 337_u32.to_le_bytes();
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
        writeln!(s, "    versions = \"3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_SPELLENERGIZELOG {}
impl crate::Message for SMSG_SPELLENERGIZELOG {
    const OPCODE: u32 = 0x0151;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(16..=30).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0151, size: body_size });
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

