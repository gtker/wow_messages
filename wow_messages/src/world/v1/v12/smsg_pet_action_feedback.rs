use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{PetFeedback, PetFeedbackError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new4.wowm:297`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new4.wowm#L297):
/// ```text
/// smsg SMSG_PET_ACTION_FEEDBACK = 0x2C6 {
///     PetFeedback feedback;
/// }
/// ```
pub struct SMSG_PET_ACTION_FEEDBACK {
    pub feedback: PetFeedback,
}

impl WorldServerMessageWrite for SMSG_PET_ACTION_FEEDBACK {
    const OPCODE: u16 = 0x2c6;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_PET_ACTION_FEEDBACK {
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

