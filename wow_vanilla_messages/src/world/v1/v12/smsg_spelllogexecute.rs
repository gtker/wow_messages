use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{SpellLog, SpellLogError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_SPELLLOGEXECUTE {
    pub caster: Guid,
    pub spell: u32,
    pub logs: Vec<SpellLog>,
}

impl ServerMessageWrite for SMSG_SPELLLOGEXECUTE {
    const OPCODE: u16 = 0x24c;

    fn size_without_size_field(&self) -> u16 {
        self.size() as u16
    }

}

impl MessageBody for SMSG_SPELLLOGEXECUTE {
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

