use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::{
    HitInfo, SpellSchool,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spellnonmeleedamagelog.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spellnonmeleedamagelog.wowm#L1):
/// ```text
/// smsg SMSG_SPELLNONMELEEDAMAGELOG = 0x0250 {
///     PackedGuid target;
///     PackedGuid attacker;
///     u32 spell;
///     u32 damage;
///     SpellSchool school;
///     u32 absorbed_damage;
///     u32 resisted;
///     Bool periodic_log;
///     u8 unused;
///     u32 blocked;
///     HitInfo hit_info;
///     u8 extend_flag;
/// }
/// ```
pub struct SMSG_SPELLNONMELEEDAMAGELOG {
    pub target: Guid,
    pub attacker: Guid,
    pub spell: u32,
    pub damage: u32,
    pub school: SpellSchool,
    pub absorbed_damage: u32,
    /// cmangos/mangoszero/vmangos: sent as int32
    ///
    pub resisted: u32,
    /// cmangos/mangoszero/vmangos: if 1, then client show spell name (example: %s's ranged shot hit %s for %u school or %s suffers %u school damage from %s's spell_name
    ///
    pub periodic_log: bool,
    pub unused: u8,
    pub blocked: u32,
    pub hit_info: HitInfo,
    /// cmangos has some that might be correct `https://github.com/cmangos/mangos-classic/blob/524a39412dae7946d06e4b8f319f45b615075815/src/game/Entities/Unit.cpp#L5497`.
    ///
    pub extend_flag: u8,
}

#[cfg(feature = "print-testcase")]
impl SMSG_SPELLNONMELEEDAMAGELOG {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SPELLNONMELEEDAMAGELOG {{").unwrap();
        // Members
        writeln!(s, "    target = {};", self.target.guid()).unwrap();
        writeln!(s, "    attacker = {};", self.attacker.guid()).unwrap();
        writeln!(s, "    spell = {};", self.spell).unwrap();
        writeln!(s, "    damage = {};", self.damage).unwrap();
        writeln!(s, "    school = {};", self.school.as_test_case_value()).unwrap();
        writeln!(s, "    absorbed_damage = {};", self.absorbed_damage).unwrap();
        writeln!(s, "    resisted = {};", self.resisted).unwrap();
        writeln!(s, "    periodic_log = {};", if self.periodic_log { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    unused = {};", self.unused).unwrap();
        writeln!(s, "    blocked = {};", self.blocked).unwrap();
        writeln!(s, "    hit_info = {};", self.hit_info.as_test_case_value()).unwrap();
        writeln!(s, "    extend_flag = {};", self.extend_flag).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 592_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.target), "target", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.attacker), "attacker", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "damage", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "school", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "absorbed_damage", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "resisted", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "periodic_log", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "unused", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "blocked", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "hit_info", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "extend_flag", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2.4.3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_SPELLNONMELEEDAMAGELOG {}
impl crate::Message for SMSG_SPELLNONMELEEDAMAGELOG {
    const OPCODE: u32 = 0x0250;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_SPELLNONMELEEDAMAGELOG::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // target: PackedGuid
        crate::util::write_packed_guid(&self.target, &mut w)?;

        // attacker: PackedGuid
        crate::util::write_packed_guid(&self.attacker, &mut w)?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // school: SpellSchool
        w.write_all(&(self.school.as_int().to_le_bytes()))?;

        // absorbed_damage: u32
        w.write_all(&self.absorbed_damage.to_le_bytes())?;

        // resisted: u32
        w.write_all(&self.resisted.to_le_bytes())?;

        // periodic_log: Bool
        w.write_all(u8::from(self.periodic_log).to_le_bytes().as_slice())?;

        // unused: u8
        w.write_all(&self.unused.to_le_bytes())?;

        // blocked: u32
        w.write_all(&self.blocked.to_le_bytes())?;

        // hit_info: HitInfo
        w.write_all(&(self.hit_info.as_int().to_le_bytes()))?;

        // extend_flag: u8
        w.write_all(&self.extend_flag.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(32..=46).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0250, size: body_size });
        }

        // target: PackedGuid
        let target = crate::util::read_packed_guid(&mut r)?;

        // attacker: PackedGuid
        let attacker = crate::util::read_packed_guid(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // damage: u32
        let damage = crate::util::read_u32_le(&mut r)?;

        // school: SpellSchool
        let school = crate::util::read_u8_le(&mut r)?.try_into()?;

        // absorbed_damage: u32
        let absorbed_damage = crate::util::read_u32_le(&mut r)?;

        // resisted: u32
        let resisted = crate::util::read_u32_le(&mut r)?;

        // periodic_log: Bool
        let periodic_log = crate::util::read_u8_le(&mut r)? != 0;

        // unused: u8
        let unused = crate::util::read_u8_le(&mut r)?;

        // blocked: u32
        let blocked = crate::util::read_u32_le(&mut r)?;

        // hit_info: HitInfo
        let hit_info = crate::util::read_u32_le(&mut r)?.try_into()?;

        // extend_flag: u8
        let extend_flag = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            target,
            attacker,
            spell,
            damage,
            school,
            absorbed_damage,
            resisted,
            periodic_log,
            unused,
            blocked,
            hit_info,
            extend_flag,
        })
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPELLNONMELEEDAMAGELOG {}

impl SMSG_SPELLNONMELEEDAMAGELOG {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.target) // target: PackedGuid
        + crate::util::packed_guid_size(&self.attacker) // attacker: PackedGuid
        + 4 // spell: u32
        + 4 // damage: u32
        + 1 // school: SpellSchool
        + 4 // absorbed_damage: u32
        + 4 // resisted: u32
        + 1 // periodic_log: Bool
        + 1 // unused: u8
        + 4 // blocked: u32
        + 4 // hit_info: HitInfo
        + 1 // extend_flag: u8
    }
}

