use std::convert::{TryFrom, TryInto};
use crate::helper::Guid;
use crate::world::v1::v12::{SpellSchool, SpellSchoolError};
use crate::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spellnonmeleedamagelog.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spellnonmeleedamagelog.wowm#L3):
/// ```text
/// smsg SMSG_SPELLNONMELEEDAMAGELOG = 0x250 {
///     PackedGuid target;
///     PackedGuid attacker;
///     u32 spell;
///     u32 damage;
///     SpellSchool school;
///     u32 absorbed_damage;
///     u32 resisted;
///     u8 periodic_log;
///     u8 unused;
///     u32 blocked;
///     u32 hit_info;
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
    pub resisted: u32,
    pub periodic_log: u8,
    pub unused: u8,
    pub blocked: u32,
    pub hit_info: u32,
    pub extend_flag: u8,
}

impl WorldServerMessageWrite for SMSG_SPELLNONMELEEDAMAGELOG {
    const OPCODE: u16 = 0x250;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_SPELLNONMELEEDAMAGELOG {
    type Error = SMSG_SPELLNONMELEEDAMAGELOGError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // target: PackedGuid
        let target = Guid::read_packed(r)?;

        // attacker: PackedGuid
        let attacker = Guid::read_packed(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // damage: u32
        let damage = crate::util::read_u32_le(r)?;

        // school: SpellSchool
        let school = SpellSchool::read(r)?;

        // absorbed_damage: u32
        let absorbed_damage = crate::util::read_u32_le(r)?;

        // resisted: u32
        let resisted = crate::util::read_u32_le(r)?;

        // periodic_log: u8
        let periodic_log = crate::util::read_u8_le(r)?;

        // unused: u8
        let unused = crate::util::read_u8_le(r)?;

        // blocked: u32
        let blocked = crate::util::read_u32_le(r)?;

        // hit_info: u32
        let hit_info = crate::util::read_u32_le(r)?;

        // extend_flag: u8
        let extend_flag = crate::util::read_u8_le(r)?;

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

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // target: PackedGuid
        self.target.write_packed(w)?;

        // attacker: PackedGuid
        self.attacker.write_packed(w)?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // school: SpellSchool
        self.school.write(w)?;

        // absorbed_damage: u32
        w.write_all(&self.absorbed_damage.to_le_bytes())?;

        // resisted: u32
        w.write_all(&self.resisted.to_le_bytes())?;

        // periodic_log: u8
        w.write_all(&self.periodic_log.to_le_bytes())?;

        // unused: u8
        w.write_all(&self.unused.to_le_bytes())?;

        // blocked: u32
        w.write_all(&self.blocked.to_le_bytes())?;

        // hit_info: u32
        w.write_all(&self.hit_info.to_le_bytes())?;

        // extend_flag: u8
        w.write_all(&self.extend_flag.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for SMSG_SPELLNONMELEEDAMAGELOG {
    fn size(&self) -> usize {
        self.target.size() // target: PackedGuid
        + self.attacker.size() // attacker: PackedGuid
        + 4 // spell: u32
        + 4 // damage: u32
        + SpellSchool::size() // school: SpellSchool
        + 4 // absorbed_damage: u32
        + 4 // resisted: u32
        + 1 // periodic_log: u8
        + 1 // unused: u8
        + 4 // blocked: u32
        + 4 // hit_info: u32
        + 1 // extend_flag: u8
    }
}

impl MaximumPossibleSized for SMSG_SPELLNONMELEEDAMAGELOG {
    fn maximum_possible_size() -> usize {
        9 // target: PackedGuid
        + 9 // attacker: PackedGuid
        + 4 // spell: u32
        + 4 // damage: u32
        + SpellSchool::maximum_possible_size() // school: SpellSchool
        + 4 // absorbed_damage: u32
        + 4 // resisted: u32
        + 1 // periodic_log: u8
        + 1 // unused: u8
        + 4 // blocked: u32
        + 4 // hit_info: u32
        + 1 // extend_flag: u8
    }
}

#[derive(Debug)]
pub enum SMSG_SPELLNONMELEEDAMAGELOGError {
    Io(std::io::Error),
    SpellSchool(SpellSchoolError),
}

impl std::error::Error for SMSG_SPELLNONMELEEDAMAGELOGError {}
impl std::fmt::Display for SMSG_SPELLNONMELEEDAMAGELOGError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SpellSchool(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_SPELLNONMELEEDAMAGELOGError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SpellSchoolError> for SMSG_SPELLNONMELEEDAMAGELOGError {
    fn from(e: SpellSchoolError) -> Self {
        Self::SpellSchool(e)
    }
}

