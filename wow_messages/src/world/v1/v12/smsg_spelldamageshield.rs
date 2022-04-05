use std::convert::{TryFrom, TryInto};
use crate::world::helper::Guid;
use crate::world::v1::v12::{SpellSchool, SpellSchoolError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new5.wowm:184`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new5.wowm#L184):
/// ```text
/// smsg SMSG_SPELLDAMAGESHIELD = 0x24F {
///     Guid victim_guid;
///     Guid caster_guid;
///     u32 damage;
///     SpellSchool school;
/// }
/// ```
pub struct SMSG_SPELLDAMAGESHIELD {
    pub victim_guid: Guid,
    pub caster_guid: Guid,
    pub damage: u32,
    pub school: SpellSchool,
}

impl WorldServerMessageWrite for SMSG_SPELLDAMAGESHIELD {
    const OPCODE: u16 = 0x24f;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_SPELLDAMAGESHIELD {
    type Error = SMSG_SPELLDAMAGESHIELDError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // victim_guid: Guid
        let victim_guid = Guid::read(r)?;

        // caster_guid: Guid
        let caster_guid = Guid::read(r)?;

        // damage: u32
        let damage = crate::util::read_u32_le(r)?;

        // school: SpellSchool
        let school = SpellSchool::read_u32_le(r)?;

        Ok(Self {
            victim_guid,
            caster_guid,
            damage,
            school,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // victim_guid: Guid
        self.victim_guid.write(w)?;

        // caster_guid: Guid
        self.caster_guid.write(w)?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // school: SpellSchool
        self.school.write_u32_le(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_SPELLDAMAGESHIELD {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_SPELLDAMAGESHIELD {
    fn maximum_possible_size() -> usize {
        8 // victim_guid: Guid
        + 8 // caster_guid: Guid
        + 4 // damage: u32
        + 4 // school: SpellSchool upcasted to u32
    }
}

#[derive(Debug)]
pub enum SMSG_SPELLDAMAGESHIELDError {
    Io(std::io::Error),
    SpellSchool(SpellSchoolError),
}

impl std::error::Error for SMSG_SPELLDAMAGESHIELDError {}
impl std::fmt::Display for SMSG_SPELLDAMAGESHIELDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SpellSchool(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_SPELLDAMAGESHIELDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SpellSchoolError> for SMSG_SPELLDAMAGESHIELDError {
    fn from(e: SpellSchoolError) -> Self {
        Self::SpellSchool(e)
    }
}

