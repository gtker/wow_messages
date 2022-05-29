use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::PartyOperation;
use crate::world::v1::v12::PartyResult;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_PARTY_COMMAND_RESULT {
    pub operation: PartyOperation,
    pub member: String,
    pub result: PartyResult,
}

impl ServerMessage for SMSG_PARTY_COMMAND_RESULT {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // operation: PartyOperation
        w.write_all(&(self.operation.as_int() as u32).to_le_bytes())?;

        // member: CString
        w.write_all(self.member.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // result: PartyResult
        w.write_all(&(self.result.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x007f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // operation: PartyOperation
        let operation: PartyOperation = (crate::util::read_u32_le(r)? as u8).try_into()?;

        // member: CString
        let member = crate::util::read_c_string_to_vec(r)?;
        let member = String::from_utf8(member)?;

        // result: PartyResult
        let result: PartyResult = (crate::util::read_u32_le(r)? as u8).try_into()?;

        Ok(Self {
            operation,
            member,
            result,
        })
    }

}

impl SMSG_PARTY_COMMAND_RESULT {
    pub(crate) fn size(&self) -> usize {
        0
        + 4 // operation: PartyOperation
        + self.member.len() + 1 // member: CString
        + 4 // result: PartyResult
    }
}

