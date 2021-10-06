use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{SpellMiss, SpellMissError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/new3.wowm:23`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/new3.wowm):
/// ```text
/// smsg SMSG_SPELLLOGMISS = 0x24B {
///     u32 spell_id;
///     u64 caster_guid;
///     u8 unknown1;
///     u32 amount_of_targets;
///     SpellMiss[amount_of_targets] targets;
/// }
/// ```
pub struct SMSG_SPELLLOGMISS {
    pub spell_id: u32,
    pub caster_guid: u64,
    pub unknown1: u8,
    pub amount_of_targets: u32,
    pub targets: Vec<SpellMiss>,
}

impl WorldServerMessageWrite for SMSG_SPELLLOGMISS {
    const OPCODE: u16 = 0x24b;

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
impl WorldMessageBody for SMSG_SPELLLOGMISS {
    type Error = SMSG_SPELLLOGMISSError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // spell_id: u32
        let spell_id = crate::util::read_u32_le(r)?;

        // caster_guid: u64
        let caster_guid = crate::util::read_u64_le(r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        // amount_of_targets: u32
        let amount_of_targets = crate::util::read_u32_le(r)?;

        // targets: SpellMiss[amount_of_targets]
        let mut targets = Vec::with_capacity(amount_of_targets as usize);
        for i in 0..amount_of_targets {
            targets.push(SpellMiss::read(r)?);
        }

        Ok(Self {
            spell_id,
            caster_guid,
            unknown1,
            amount_of_targets,
            targets,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // spell_id: u32
        w.write_all(&self.spell_id.to_le_bytes())?;

        // caster_guid: u64
        w.write_all(&self.caster_guid.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // amount_of_targets: u32
        w.write_all(&(self.targets.len() as u32).to_le_bytes())?;

        // targets: SpellMiss[amount_of_targets]
        for i in self.targets.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_SPELLLOGMISS {
    fn size(&self) -> usize {
        4 // spell_id: u32
        + 8 // caster_guid: u64
        + 1 // unknown1: u8
        + 4 // amount_of_targets: u32
        + self.targets.iter().fold(0, |acc, x| acc + SpellMiss::size()) // targets: SpellMiss[amount_of_targets]
    }
}

impl MaximumPossibleSized for SMSG_SPELLLOGMISS {
    fn maximum_possible_size() -> usize {
        4 // spell_id: u32
        + 8 // caster_guid: u64
        + 1 // unknown1: u8
        + 4 // amount_of_targets: u32
        + 4294967295 * SpellMiss::maximum_possible_size() // targets: SpellMiss[amount_of_targets]
    }
}

#[derive(Debug)]
pub enum SMSG_SPELLLOGMISSError {
    Io(std::io::Error),
    SpellMiss(SpellMissError),
}

impl std::error::Error for SMSG_SPELLLOGMISSError {}
impl std::fmt::Display for SMSG_SPELLLOGMISSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SpellMiss(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_SPELLLOGMISSError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SpellMissError> for SMSG_SPELLLOGMISSError {
    fn from(e: SpellMissError) -> Self {
        Self::SpellMiss(e)
    }
}

