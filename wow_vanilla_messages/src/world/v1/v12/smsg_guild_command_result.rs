use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::GuildCommand;
use crate::world::v1::v12::GuildCommandResult;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_GUILD_COMMAND_RESULT {
    pub command: GuildCommand,
    pub string: String,
    pub result: GuildCommandResult,
}

impl ServerMessage for SMSG_GUILD_COMMAND_RESULT {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // command: GuildCommand
        w.write_all(&(self.command.as_int() as u32).to_le_bytes())?;

        // string: CString
        w.write_all(self.string.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // result: GuildCommandResult
        w.write_all(&(self.result.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0093;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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

impl SMSG_GUILD_COMMAND_RESULT {
    pub fn size(&self) -> usize {
        0
        + 4 // command: GuildCommand
        + self.string.len() + 1 // string: CString
        + 4 // result: GuildCommandResult
    }
}

