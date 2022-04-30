use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{PetTameFailureReason, PetTameFailureReasonError};
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
pub struct SMSG_PET_TAME_FAILURE {
    pub reason: PetTameFailureReason,
}

impl ServerMessageWrite for SMSG_PET_TAME_FAILURE {
    const OPCODE: u16 = 0x173;

    fn size_without_size_field(&self) -> u16 {
        Self::size() as u16
    }

}

impl MessageBody for SMSG_PET_TAME_FAILURE {
    type Error = SMSG_PET_TAME_FAILUREError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // reason: PetTameFailureReason
        let reason = PetTameFailureReason::read(r)?;

        Ok(Self {
            reason,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // reason: PetTameFailureReason
        self.reason.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_PET_TAME_FAILURE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_PET_TAME_FAILURE {
    fn maximum_possible_size() -> usize {
        PetTameFailureReason::size() // reason: PetTameFailureReason
    }
}

#[derive(Debug)]
pub enum SMSG_PET_TAME_FAILUREError {
    Io(std::io::Error),
    PetTameFailureReason(PetTameFailureReasonError),
}

impl std::error::Error for SMSG_PET_TAME_FAILUREError {}
impl std::fmt::Display for SMSG_PET_TAME_FAILUREError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::PetTameFailureReason(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PET_TAME_FAILUREError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<PetTameFailureReasonError> for SMSG_PET_TAME_FAILUREError {
    fn from(e: PetTameFailureReasonError) -> Self {
        Self::PetTameFailureReason(e)
    }
}

