use std::convert::{TryFrom, TryInto};
use crate::world::helper::Guid;
use crate::world::v1::v12::{SpellLog, SpellLogError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/needs_else_if_else.wowm:690`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/needs_else_if_else.wowm#L690):
/// ```text
/// smsg SMSG_SPELLLOGEXECUTE = 0x24C {
///     PackedGuid caster;
///     u32 spell;
///     u32 amount_of_effects;
///     SpellLog[amount_of_effects] logs;
/// }
/// ```
pub struct SMSG_SPELLLOGEXECUTE {
    pub caster: Guid,
    pub spell: u32,
    pub amount_of_effects: u32,
    pub logs: Vec<SpellLog>,
}

impl WorldServerMessageWrite for SMSG_SPELLLOGEXECUTE {
    const OPCODE: u16 = 0x24c;

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
impl WorldMessageBody for SMSG_SPELLLOGEXECUTE {
    type Error = SMSG_SPELLLOGEXECUTEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // caster: PackedGuid
        let caster = Guid::read_packed(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // amount_of_effects: u32
        let amount_of_effects = crate::util::read_u32_le(r)?;

        // logs: SpellLog[amount_of_effects]
        let mut logs = Vec::with_capacity(amount_of_effects as usize);
        for i in 0..amount_of_effects {
            logs.push(SpellLog::read(r)?);
        }

        Ok(Self {
            caster,
            spell,
            amount_of_effects,
            logs,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // caster: PackedGuid
        self.caster.write_packed(w)?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // amount_of_effects: u32
        w.write_all(&(self.logs.len() as u32).to_le_bytes())?;

        // logs: SpellLog[amount_of_effects]
        for i in self.logs.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_SPELLLOGEXECUTE {
    fn size(&self) -> usize {
        self.caster.size() // caster: PackedGuid
        + 4 // spell: u32
        + 4 // amount_of_effects: u32
        + self.logs.iter().fold(0, |acc, x| acc + x.size()) // logs: SpellLog[amount_of_effects]
    }
}

impl MaximumPossibleSized for SMSG_SPELLLOGEXECUTE {
    fn maximum_possible_size() -> usize {
        9 // caster: PackedGuid
        + 4 // spell: u32
        + 4 // amount_of_effects: u32
        + 4294967295 * SpellLog::maximum_possible_size() // logs: SpellLog[amount_of_effects]
    }
}

#[derive(Debug)]
pub enum SMSG_SPELLLOGEXECUTEError {
    Io(std::io::Error),
    SpellLog(SpellLogError),
}

impl std::error::Error for SMSG_SPELLLOGEXECUTEError {}
impl std::fmt::Display for SMSG_SPELLLOGEXECUTEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SpellLog(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_SPELLLOGEXECUTEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SpellLogError> for SMSG_SPELLLOGEXECUTEError {
    fn from(e: SpellLogError) -> Self {
        Self::SpellLog(e)
    }
}

