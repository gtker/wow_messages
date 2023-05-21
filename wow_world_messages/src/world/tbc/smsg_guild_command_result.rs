use std::io::{Read, Write};

use crate::tbc::{
    GuildCommand, GuildCommandResult,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_guild_command_result.wowm:114`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_command_result.wowm#L114):
/// ```text
/// smsg SMSG_GUILD_COMMAND_RESULT = 0x0093 {
///     (u32)GuildCommand command;
///     CString string;
///     (u32)GuildCommandResult result;
/// }
/// ```
pub struct SMSG_GUILD_COMMAND_RESULT {
    pub command: GuildCommand,
    pub string: String,
    pub result: GuildCommandResult,
}

impl crate::private::Sealed for SMSG_GUILD_COMMAND_RESULT {}
impl SMSG_GUILD_COMMAND_RESULT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(9..=264).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // command: GuildCommand
        let command = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // string: CString
        let string = {
            let string = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(string)?
        };

        // result: GuildCommandResult
        let result = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        Ok(Self {
            command,
            string,
            result,
        })
    }

}

impl crate::Message for SMSG_GUILD_COMMAND_RESULT {
    const OPCODE: u32 = 0x0093;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_GUILD_COMMAND_RESULT {{").unwrap();
        // Members
        writeln!(s, "    command = {};", self.command.as_test_case_value()).unwrap();
        writeln!(s, "    string = \"{}\";", self.string).unwrap();
        writeln!(s, "    result = {};", self.result.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 147_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "command", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.string.len() + 1, "string", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "result", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // command: GuildCommand
        w.write_all(&u32::from(self.command.as_int()).to_le_bytes())?;

        // string: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.string.as_bytes().iter().rev().next(), Some(&0_u8), "String `string` must not be null-terminated.");
        w.write_all(self.string.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // result: GuildCommandResult
        w.write_all(&u32::from(self.result.as_int()).to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(147, "SMSG_GUILD_COMMAND_RESULT", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_GUILD_COMMAND_RESULT {}

impl SMSG_GUILD_COMMAND_RESULT {
    pub(crate) fn size(&self) -> usize {
        4 // command: GuildCommand
        + self.string.len() + 1 // string: CString
        + 4 // result: GuildCommandResult
    }
}

