use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{SpellMiss, SpellMissError};
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
pub struct SMSG_SPELLLOGMISS {
    pub id: u32,
    pub caster_guid: Guid,
    pub unknown1: u8,
    pub targets: Vec<SpellMiss>,
}

impl ServerMessageWrite for SMSG_SPELLLOGMISS {
    const OPCODE: u16 = 0x24b;

    fn size_without_size_field(&self) -> u16 {
        self.size() as u16
    }

}

impl MessageBody for SMSG_SPELLLOGMISS {
    type Error = SMSG_SPELLLOGMISSError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // caster_guid: Guid
        let caster_guid = Guid::read(r)?;

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
            id,
            caster_guid,
            unknown1,
            targets,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // caster_guid: Guid
        self.caster_guid.write(w)?;

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
        4 // id: u32
        + 8 // caster_guid: Guid
        + 1 // unknown1: u8
        + 4 // amount_of_targets: u32
        + self.targets.iter().fold(0, |acc, x| acc + SpellMiss::size()) // targets: SpellMiss[amount_of_targets]
    }
}

impl MaximumPossibleSized for SMSG_SPELLLOGMISS {
    fn maximum_possible_size() -> usize {
        4 // id: u32
        + 8 // caster_guid: Guid
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

