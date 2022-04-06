use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{PartyOperation, PartyOperationError};
use crate::world::v1::v12::{PartyResult, PartyResultError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_party_command_result.wowm:20`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_party_command_result.wowm#L20):
/// ```text
/// smsg SMSG_PARTY_COMMAND_RESULT = 0x7F {
///     PartyOperation operation;
///     CString member;
///     PartyResult result;
/// }
/// ```
pub struct SMSG_PARTY_COMMAND_RESULT {
    pub operation: PartyOperation,
    pub member: String,
    pub result: PartyResult,
}

impl WorldServerMessageWrite for SMSG_PARTY_COMMAND_RESULT {
    const OPCODE: u16 = 0x7f;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_PARTY_COMMAND_RESULT {
    type Error = SMSG_PARTY_COMMAND_RESULTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // operation: PartyOperation
        let operation = PartyOperation::read_u32_le(r)?;

        // member: CString
        let member = crate::util::read_c_string_to_vec(r)?;
        let member = String::from_utf8(member)?;

        // result: PartyResult
        let result = PartyResult::read_u32_le(r)?;

        Ok(Self {
            operation,
            member,
            result,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // operation: PartyOperation
        self.operation.write_u32_le(w)?;

        // member: CString
        w.write_all(self.member.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // result: PartyResult
        self.result.write_u32_le(w)?;

        Ok(())
    }
}

impl VariableSized for SMSG_PARTY_COMMAND_RESULT {
    fn size(&self) -> usize {
        4 // operation: PartyOperation upcasted to u32
        + self.member.len() + 1 // member: CString and Null Terminator
        + 4 // result: PartyResult upcasted to u32
    }
}

impl MaximumPossibleSized for SMSG_PARTY_COMMAND_RESULT {
    fn maximum_possible_size() -> usize {
        PartyOperation::maximum_possible_size() // operation: PartyOperation
        + 256 // member: CString
        + PartyResult::maximum_possible_size() // result: PartyResult
    }
}

#[derive(Debug)]
pub enum SMSG_PARTY_COMMAND_RESULTError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    PartyOperation(PartyOperationError),
    PartyResult(PartyResultError),
}

impl std::error::Error for SMSG_PARTY_COMMAND_RESULTError {}
impl std::fmt::Display for SMSG_PARTY_COMMAND_RESULTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::PartyOperation(i) => i.fmt(f),
            Self::PartyResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PARTY_COMMAND_RESULTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_PARTY_COMMAND_RESULTError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<PartyOperationError> for SMSG_PARTY_COMMAND_RESULTError {
    fn from(e: PartyOperationError) -> Self {
        Self::PartyOperation(e)
    }
}

impl From<PartyResultError> for SMSG_PARTY_COMMAND_RESULTError {
    fn from(e: PartyResultError) -> Self {
        Self::PartyResult(e)
    }
}

