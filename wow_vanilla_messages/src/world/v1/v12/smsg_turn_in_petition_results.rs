use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{PetitionTurnInResult, PetitionTurnInResultError};
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
pub struct SMSG_TURN_IN_PETITION_RESULTS {
    pub result: PetitionTurnInResult,
}

impl ServerMessageWrite for SMSG_TURN_IN_PETITION_RESULTS {
    const OPCODE: u16 = 0x1c5;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

}

impl MessageBody for SMSG_TURN_IN_PETITION_RESULTS {
    type Error = SMSG_TURN_IN_PETITION_RESULTSError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // result: PetitionTurnInResult
        let result = PetitionTurnInResult::read(r)?;

        Ok(Self {
            result,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // result: PetitionTurnInResult
        self.result.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_TURN_IN_PETITION_RESULTS {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_TURN_IN_PETITION_RESULTS {
    fn maximum_possible_size() -> usize {
        PetitionTurnInResult::size() // result: PetitionTurnInResult
    }
}

#[derive(Debug)]
pub enum SMSG_TURN_IN_PETITION_RESULTSError {
    Io(std::io::Error),
    PetitionTurnInResult(PetitionTurnInResultError),
}

impl std::error::Error for SMSG_TURN_IN_PETITION_RESULTSError {}
impl std::fmt::Display for SMSG_TURN_IN_PETITION_RESULTSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::PetitionTurnInResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_TURN_IN_PETITION_RESULTSError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<PetitionTurnInResultError> for SMSG_TURN_IN_PETITION_RESULTSError {
    fn from(e: PetitionTurnInResultError) -> Self {
        Self::PetitionTurnInResult(e)
    }
}

