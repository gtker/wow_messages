use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{SpellCastTargets, SpellCastTargetsError};
use crate::{ClientMessageWrite, MessageBody};
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
pub struct CMSG_CAST_SPELL {
    pub spell: u32,
    pub targets: SpellCastTargets,
}

impl ClientMessageWrite for CMSG_CAST_SPELL {
    const OPCODE: u32 = 0x12e;

    fn size_without_size_field(&self) -> u16 {
        self.size() as u16
    }

}

impl MessageBody for CMSG_CAST_SPELL {
    type Error = CMSG_CAST_SPELLError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // targets: SpellCastTargets
        let targets = SpellCastTargets::read(r)?;

        Ok(Self {
            spell,
            targets,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // targets: SpellCastTargets
        self.targets.write(w)?;

        Ok(())
    }
}

impl VariableSized for CMSG_CAST_SPELL {
    fn size(&self) -> usize {
        4 // spell: u32
        + self.targets.size() // targets: SpellCastTargets
    }
}

impl MaximumPossibleSized for CMSG_CAST_SPELL {
    fn maximum_possible_size() -> usize {
        4 // spell: u32
        + SpellCastTargets::maximum_possible_size() // targets: SpellCastTargets
    }
}

#[derive(Debug)]
pub enum CMSG_CAST_SPELLError {
    Io(std::io::Error),
    SpellCastTargets(SpellCastTargetsError),
}

impl std::error::Error for CMSG_CAST_SPELLError {}
impl std::fmt::Display for CMSG_CAST_SPELLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SpellCastTargets(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_CAST_SPELLError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SpellCastTargetsError> for CMSG_CAST_SPELLError {
    fn from(e: SpellCastTargetsError) -> Self {
        Self::SpellCastTargets(e)
    }
}

