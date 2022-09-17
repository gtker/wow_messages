use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::GuildCommand;
use crate::world::vanilla::GuildCommandResult;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_guild_command_result.wowm:38`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_command_result.wowm#L38):
/// ```text
/// smsg SMSG_GUILD_COMMAND_RESULT = 0x0093 {
///     GuildCommand command;
///     CString string;
///     GuildCommandResult result;
/// }
/// ```
pub struct SMSG_GUILD_COMMAND_RESULT {
    pub command: GuildCommand,
    pub string: String,
    pub result: GuildCommandResult,
}

impl crate::Message for SMSG_GUILD_COMMAND_RESULT {
    const OPCODE: u32 = 0x0093;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // command: GuildCommand
        w.write_all(&(self.command.as_int() as u32).to_le_bytes())?;

        // string: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.string.as_bytes().iter().rev().next(), Some(&0_u8), "String `string` must not be null-terminated.");
        w.write_all(self.string.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // result: GuildCommandResult
        w.write_all(&(self.result.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // command: GuildCommand
        let command: GuildCommand = (crate::util::read_u32_le(r)? as u8).try_into()?;

        // string: CString
        let string = crate::util::read_c_string_to_vec(r)?;
        let string = String::from_utf8(string)?;

        // result: GuildCommandResult
        let result: GuildCommandResult = (crate::util::read_u32_le(r)? as u8).try_into()?;

        Ok(Self {
            command,
            string,
            result,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_GUILD_COMMAND_RESULT {}

impl SMSG_GUILD_COMMAND_RESULT {
    pub(crate) fn size(&self) -> usize {
        4 // command: GuildCommand
        + self.string.len() + 1 // string: CString
        + 4 // result: GuildCommandResult
    }
}

