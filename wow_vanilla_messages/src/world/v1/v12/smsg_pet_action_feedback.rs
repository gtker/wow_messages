use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{PetFeedback, PetFeedbackError};
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
pub struct SMSG_PET_ACTION_FEEDBACK {
    pub feedback: PetFeedback,
}

impl ServerMessageWrite for SMSG_PET_ACTION_FEEDBACK {}

impl MessageBody for SMSG_PET_ACTION_FEEDBACK {
    const OPCODE: u16 = 0x02c6;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_PET_ACTION_FEEDBACKError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // feedback: PetFeedback
        let feedback = PetFeedback::read(r)?;

        Ok(Self {
            feedback,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // feedback: PetFeedback
        self.feedback.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_PET_ACTION_FEEDBACK {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_PET_ACTION_FEEDBACK {
    fn maximum_possible_size() -> usize {
        PetFeedback::size() // feedback: PetFeedback
    }
}

#[derive(Debug)]
pub enum SMSG_PET_ACTION_FEEDBACKError {
    Io(std::io::Error),
    PetFeedback(PetFeedbackError),
}

impl std::error::Error for SMSG_PET_ACTION_FEEDBACKError {}
impl std::fmt::Display for SMSG_PET_ACTION_FEEDBACKError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::PetFeedback(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PET_ACTION_FEEDBACKError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<PetFeedbackError> for SMSG_PET_ACTION_FEEDBACKError {
    fn from(e: PetFeedbackError) -> Self {
        Self::PetFeedback(e)
    }
}

