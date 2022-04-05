use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{PetTameFailureReason, PetTameFailureReasonError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new_all.wowm:2169`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new_all.wowm#L2169):
/// ```text
/// smsg SMSG_PET_TAME_FAILURE = 0x173 {
///     PetTameFailureReason reason;
/// }
/// ```
pub struct SMSG_PET_TAME_FAILURE {
    pub reason: PetTameFailureReason,
}

impl WorldServerMessageWrite for SMSG_PET_TAME_FAILURE {
    const OPCODE: u16 = 0x173;

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
impl WorldMessageBody for SMSG_PET_TAME_FAILURE {
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

