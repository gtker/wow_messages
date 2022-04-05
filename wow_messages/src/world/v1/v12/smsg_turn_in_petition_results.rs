use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{PetitionTurnInResult, PetitionTurnInResultError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new2.wowm:437`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new2.wowm#L437):
/// ```text
/// smsg SMSG_TURN_IN_PETITION_RESULTS = 0x1C5 {
///     PetitionTurnInResult result;
/// }
/// ```
pub struct SMSG_TURN_IN_PETITION_RESULTS {
    pub result: PetitionTurnInResult,
}

impl WorldServerMessageWrite for SMSG_TURN_IN_PETITION_RESULTS {
    const OPCODE: u16 = 0x1c5;

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
impl WorldMessageBody for SMSG_TURN_IN_PETITION_RESULTS {
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

