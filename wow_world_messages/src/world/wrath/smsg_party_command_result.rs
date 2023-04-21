use std::io::{Read, Write};

use crate::wrath::{
    PartyOperation, PartyResult,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_party_command_result.wowm:49`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_party_command_result.wowm#L49):
/// ```text
/// smsg SMSG_PARTY_COMMAND_RESULT = 0x007F {
///     (u32)PartyOperation operation;
///     CString member;
///     (u32)PartyResult result;
/// }
/// ```
pub struct SMSG_PARTY_COMMAND_RESULT {
    pub operation: PartyOperation,
    pub member: String,
    pub result: PartyResult,
}

impl crate::private::Sealed for SMSG_PARTY_COMMAND_RESULT {}
impl crate::Message for SMSG_PARTY_COMMAND_RESULT {
    const OPCODE: u32 = 0x007f;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // operation: PartyOperation
        w.write_all(&u32::from(self.operation.as_int()).to_le_bytes())?;

        // member: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.member.as_bytes().iter().rev().next(), Some(&0_u8), "String `member` must not be null-terminated.");
        w.write_all(self.member.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // result: PartyResult
        w.write_all(&u32::from(self.result.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(9..=264).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x007F, size: body_size as u32 });
        }

        // operation: PartyOperation
        let operation: PartyOperation = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // member: CString
        let member = {
            let member = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(member)?
        };

        // result: PartyResult
        let result: PartyResult = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        Ok(Self {
            operation,
            member,
            result,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PARTY_COMMAND_RESULT {}

impl SMSG_PARTY_COMMAND_RESULT {
    pub(crate) fn size(&self) -> usize {
        4 // operation: PartyOperation
        + self.member.len() + 1 // member: CString
        + 4 // result: PartyResult
    }
}

