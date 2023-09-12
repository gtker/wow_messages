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
impl SMSG_PARTY_COMMAND_RESULT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(9..=264).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // operation: PartyOperation
        let operation = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // member: CString
        let member = {
            let member = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(member)?
        };

        // result: PartyResult
        let result = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        Ok(Self {
            operation,
            member,
            result,
        })
    }

}

impl crate::Message for SMSG_PARTY_COMMAND_RESULT {
    const OPCODE: u32 = 0x007f;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_PARTY_COMMAND_RESULT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_PARTY_COMMAND_RESULT {{").unwrap();
        // Members
        writeln!(s, "    operation = {};", self.operation.as_test_case_value()).unwrap();
        writeln!(s, "    member = \"{}\";", self.member).unwrap();
        writeln!(s, "    result = {};", self.result.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 127_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "operation", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.member.len() + 1, "member", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "result", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // operation: PartyOperation
        w.write_all(&u32::from(self.operation.as_int()).to_le_bytes())?;

        // member: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.member.as_bytes().iter().next_back(), Some(&0_u8), "String `member` must not be null-terminated.");
        w.write_all(self.member.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // result: PartyResult
        w.write_all(&u32::from(self.result.as_int()).to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(127, "SMSG_PARTY_COMMAND_RESULT", body_size, a))
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

