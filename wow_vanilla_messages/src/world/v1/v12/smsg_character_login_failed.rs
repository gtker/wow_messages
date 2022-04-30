use std::convert::{TryFrom, TryInto};
use crate::world::v1::v2::{WorldResult, WorldResultError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_CHARACTER_LOGIN_FAILED {
    pub result: WorldResult,
}

impl ServerMessageWrite for SMSG_CHARACTER_LOGIN_FAILED {}

impl MessageBody for SMSG_CHARACTER_LOGIN_FAILED {
    const OPCODE: u16 = 0x0041;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_CHARACTER_LOGIN_FAILEDError;

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

impl ConstantSized for SMSG_CHARACTER_LOGIN_FAILED {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_CHARACTER_LOGIN_FAILED {
    fn maximum_possible_size() -> usize {
        WorldResult::size() // result: WorldResult
    }
}

#[derive(Debug)]
pub enum SMSG_CHARACTER_LOGIN_FAILEDError {
    Io(std::io::Error),
    WorldResult(WorldResultError),
}

impl std::error::Error for SMSG_CHARACTER_LOGIN_FAILEDError {}
impl std::fmt::Display for SMSG_CHARACTER_LOGIN_FAILEDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::WorldResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_CHARACTER_LOGIN_FAILEDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<WorldResultError> for SMSG_CHARACTER_LOGIN_FAILEDError {
    fn from(e: WorldResultError) -> Self {
        Self::WorldResult(e)
    }
}

