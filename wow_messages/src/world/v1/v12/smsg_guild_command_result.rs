use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{GuildCommand, GuildCommandError};
use crate::world::v1::v12::{GuildCommandResult, GuildCommandResultError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/add_messages.wowm:500`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/add_messages.wowm#L500):
/// ```text
/// smsg SMSG_GUILD_COMMAND_RESULT = 0x93 {
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

impl WorldServerMessageWrite for SMSG_GUILD_COMMAND_RESULT {
    const OPCODE: u16 = 0x93;

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
impl WorldMessageBody for SMSG_GUILD_COMMAND_RESULT {
    type Error = SMSG_GUILD_COMMAND_RESULTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // command: GuildCommand
        let command = GuildCommand::read_u32_le(r)?;

        // string: CString
        let string = crate::util::read_c_string_to_vec(r)?;
        let string = String::from_utf8(string)?;

        // result: GuildCommandResult
        let result = GuildCommandResult::read_u32_le(r)?;

        Ok(Self {
            command,
            string,
            result,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // command: GuildCommand
        self.command.write_u32_le(w)?;

        // string: CString
        w.write_all(self.string.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // result: GuildCommandResult
        self.result.write_u32_le(w)?;

        Ok(())
    }
}

impl VariableSized for SMSG_GUILD_COMMAND_RESULT {
    fn size(&self) -> usize {
        4 // command: GuildCommand upcasted to u32
        + self.string.len() + 1 // string: CString and Null Terminator
        + 4 // result: GuildCommandResult upcasted to u32
    }
}

impl MaximumPossibleSized for SMSG_GUILD_COMMAND_RESULT {
    fn maximum_possible_size() -> usize {
        GuildCommand::maximum_possible_size() // command: GuildCommand
        + 256 // string: CString
        + GuildCommandResult::maximum_possible_size() // result: GuildCommandResult
    }
}

#[derive(Debug)]
pub enum SMSG_GUILD_COMMAND_RESULTError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    GuildCommand(GuildCommandError),
    GuildCommandResult(GuildCommandResultError),
}

impl std::error::Error for SMSG_GUILD_COMMAND_RESULTError {}
impl std::fmt::Display for SMSG_GUILD_COMMAND_RESULTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::GuildCommand(i) => i.fmt(f),
            Self::GuildCommandResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_GUILD_COMMAND_RESULTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_GUILD_COMMAND_RESULTError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<GuildCommandError> for SMSG_GUILD_COMMAND_RESULTError {
    fn from(e: GuildCommandError) -> Self {
        Self::GuildCommand(e)
    }
}

impl From<GuildCommandResultError> for SMSG_GUILD_COMMAND_RESULTError {
    fn from(e: GuildCommandResultError) -> Self {
        Self::GuildCommandResult(e)
    }
}

