use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spellheallog.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spellheallog.wowm#L11):
/// ```text
/// smsg SMSG_SPELLHEALLOG = 0x0150 {
///     PackedGuid victim;
///     PackedGuid caster;
///     u32 id;
///     u32 damage;
///     Bool critical;
///     u8 unknown;
/// }
/// ```
pub struct SMSG_SPELLHEALLOG {
    pub victim: Guid,
    pub caster: Guid,
    pub id: u32,
    pub damage: u32,
    pub critical: bool,
    /// mangosone: unused in client?
    pub unknown: u8,
}

impl crate::private::Sealed for SMSG_SPELLHEALLOG {}
impl SMSG_SPELLHEALLOG {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(14..=28).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x0150, size: body_size });
        }

        // victim: PackedGuid
        let victim = crate::util::read_packed_guid(&mut r)?;

        // caster: PackedGuid
        let caster = crate::util::read_packed_guid(&mut r)?;

        // id: u32
        let id = crate::util::read_u32_le(&mut r)?;

        // damage: u32
        let damage = crate::util::read_u32_le(&mut r)?;

        // critical: Bool
        let critical = crate::util::read_u8_le(&mut r)? != 0;

        // unknown: u8
        let unknown = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            victim,
            caster,
            id,
            damage,
            critical,
            unknown,
        })
    }

}

impl crate::Message for SMSG_SPELLHEALLOG {
    const OPCODE: u32 = 0x0150;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SPELLHEALLOG {{").unwrap();
        // Members
        writeln!(s, "    victim = {};", self.victim.guid()).unwrap();
        writeln!(s, "    caster = {};", self.caster.guid()).unwrap();
        writeln!(s, "    id = {};", self.id).unwrap();
        writeln!(s, "    damage = {};", self.damage).unwrap();
        writeln!(s, "    critical = {};", if self.critical { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    unknown = {};", self.unknown).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 336_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.victim), "victim", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.caster), "caster", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "damage", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "critical", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
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

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // critical: Bool
        w.write_all(u8::from(self.critical).to_le_bytes().as_slice())?;

        // unknown: u8
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPELLHEALLOG {}

impl SMSG_SPELLHEALLOG {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.victim) // victim: PackedGuid
        + crate::util::packed_guid_size(&self.caster) // caster: PackedGuid
        + 4 // id: u32
        + 4 // damage: u32
        + 1 // critical: Bool
        + 1 // unknown: u8
    }
}

