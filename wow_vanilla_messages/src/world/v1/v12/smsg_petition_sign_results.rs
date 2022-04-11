use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{PetitionResult, PetitionResultError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_PETITION_SIGN_RESULTS {
    pub petition_guid: Guid,
    pub owner_guid: Guid,
    pub result: PetitionResult,
}

impl WorldServerMessageWrite for SMSG_PETITION_SIGN_RESULTS {
    const OPCODE: u16 = 0x1c1;

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
impl WorldMessageBody for SMSG_PETITION_SIGN_RESULTS {
    type Error = SMSG_PETITION_SIGN_RESULTSError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // petition_guid: Guid
        let petition_guid = Guid::read(r)?;

        // owner_guid: Guid
        let owner_guid = Guid::read(r)?;

        // result: PetitionResult
        let result = PetitionResult::read(r)?;

        Ok(Self {
            petition_guid,
            owner_guid,
            result,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // petition_guid: Guid
        self.petition_guid.write(w)?;

        // owner_guid: Guid
        self.owner_guid.write(w)?;

        // result: PetitionResult
        self.result.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_PETITION_SIGN_RESULTS {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_PETITION_SIGN_RESULTS {
    fn maximum_possible_size() -> usize {
        8 // petition_guid: Guid
        + 8 // owner_guid: Guid
        + PetitionResult::size() // result: PetitionResult
    }
}

#[derive(Debug)]
pub enum SMSG_PETITION_SIGN_RESULTSError {
    Io(std::io::Error),
    PetitionResult(PetitionResultError),
}

impl std::error::Error for SMSG_PETITION_SIGN_RESULTSError {}
impl std::fmt::Display for SMSG_PETITION_SIGN_RESULTSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::PetitionResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PETITION_SIGN_RESULTSError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<PetitionResultError> for SMSG_PETITION_SIGN_RESULTSError {
    fn from(e: PetitionResultError) -> Self {
        Self::PetitionResult(e)
    }
}

