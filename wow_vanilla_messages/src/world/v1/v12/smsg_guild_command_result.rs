use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{GuildCommand, GuildCommandError};
use crate::world::v1::v12::{GuildCommandResult, GuildCommandResultError};
use crate::{ServerMessageWrite, MessageBody};
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

impl ServerMessageWrite for SMSG_GUILD_COMMAND_RESULT {}

impl SMSG_GUILD_COMMAND_RESULT {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8000);
        // command: GuildCommand
        w.write_all(&(self.command.as_int() as u32).to_le_bytes())?;

        // string: CString
        w.write_all(self.string.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // result: GuildCommandResult
        w.write_all(&(self.result.as_int() as u32).to_le_bytes())?;

        Ok(w)
    }
}

impl MessageBody for SMSG_GUILD_COMMAND_RESULT {
    const OPCODE: u16 = 0x0093;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_GUILD_COMMAND_RESULTError;

    #[cfg(feature = "sync")]
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

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let inner = self.as_bytes()?;
        w.write_all(&inner)
    }

    #[cfg(feature = "tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // command: GuildCommand
            let command: GuildCommand = (crate::util::tokio_read_u32_le(r).await? as u8).try_into()?;

            // string: CString
            let string = crate::util::tokio_read_c_string_to_vec(r).await?;
            let string = String::from_utf8(string)?;

            // result: GuildCommandResult
            let result: GuildCommandResult = (crate::util::tokio_read_u32_le(r).await? as u8).try_into()?;

            Ok(Self {
                command,
                string,
                result,
            })
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // command: GuildCommand
            let command: GuildCommand = (crate::util::astd_read_u32_le(r).await? as u8).try_into()?;

            // string: CString
            let string = crate::util::astd_read_c_string_to_vec(r).await?;
            let string = String::from_utf8(string)?;

            // result: GuildCommandResult
            let result: GuildCommandResult = (crate::util::astd_read_u32_le(r).await? as u8).try_into()?;

            Ok(Self {
                command,
                string,
                result,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
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

