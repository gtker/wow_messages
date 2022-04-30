use std::convert::{TryFrom, TryInto};
use crate::world::v1::v2::{WorldResult, WorldResultError};
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
#[derive(Copy)]
pub struct SMSG_CHAR_CREATE {
    pub result: WorldResult,
}

impl ServerMessageWrite for SMSG_CHAR_CREATE {}

impl MessageBody for SMSG_CHAR_CREATE {
    const OPCODE: u16 = 0x003a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_CHAR_CREATEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // result: WorldResult
        let result = WorldResult::read(r)?;

        Ok(Self {
            result,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // result: WorldResult
        self.result.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_CHAR_CREATE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_CHAR_CREATE {
    fn maximum_possible_size() -> usize {
        WorldResult::size() // result: WorldResult
    }
}

#[derive(Debug)]
pub enum SMSG_CHAR_CREATEError {
    Io(std::io::Error),
    WorldResult(WorldResultError),
}

impl std::error::Error for SMSG_CHAR_CREATEError {}
impl std::fmt::Display for SMSG_CHAR_CREATEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::WorldResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_CHAR_CREATEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<WorldResultError> for SMSG_CHAR_CREATEError {
    fn from(e: WorldResultError) -> Self {
        Self::WorldResult(e)
    }
}

