use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::PartyOperation;
use crate::world::version_1_12::PartyResult;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_party_command_result.wowm:20`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_party_command_result.wowm#L20):
/// ```text
/// smsg SMSG_PARTY_COMMAND_RESULT = 0x007F {
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

    fn server_size(&self) -> u16 {
        (self.size() + 4) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
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
        4 // operation: PartyOperation
        + self.member.len() + 1 // member: CString
        + 4 // result: PartyResult
    }
}

