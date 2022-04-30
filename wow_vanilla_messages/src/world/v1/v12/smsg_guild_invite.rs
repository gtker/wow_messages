use std::convert::{TryFrom, TryInto};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_GUILD_INVITE {
    pub player_name: String,
    pub guild_name: String,
}

impl ServerMessageWrite for SMSG_GUILD_INVITE {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_GUILD_INVITE {
    const OPCODE: u16 = 0x0083;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_GUILD_INVITEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // player_name: CString
        let player_name = crate::util::read_c_string_to_vec(r)?;
        let player_name = String::from_utf8(player_name)?;

        // guild_name: CString
        let guild_name = crate::util::read_c_string_to_vec(r)?;
        let guild_name = String::from_utf8(guild_name)?;

        Ok(Self {
            player_name,
            guild_name,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // player_name: CString
        w.write_all(self.player_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // guild_name: CString
        w.write_all(self.guild_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // player_name: CString
        let player_name = crate::util::tokio_read_c_string_to_vec(r).await?;
        let player_name = String::from_utf8(player_name)?;

        // guild_name: CString
        let guild_name = crate::util::tokio_read_c_string_to_vec(r).await?;
        let guild_name = String::from_utf8(guild_name)?;

        Ok(Self {
            player_name,
            guild_name,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // player_name: CString
        w.write_all(self.player_name.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        // guild_name: CString
        w.write_all(self.guild_name.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // player_name: CString
        let player_name = crate::util::astd_read_c_string_to_vec(r).await?;
        let player_name = String::from_utf8(player_name)?;

        // guild_name: CString
        let guild_name = crate::util::astd_read_c_string_to_vec(r).await?;
        let guild_name = String::from_utf8(guild_name)?;

        Ok(Self {
            player_name,
            guild_name,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // player_name: CString
        w.write_all(self.player_name.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        // guild_name: CString
        w.write_all(self.guild_name.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        Ok(())
    }

}

impl VariableSized for SMSG_GUILD_INVITE {
    fn size(&self) -> usize {
        self.player_name.len() + 1 // player_name: CString and Null Terminator
        + self.guild_name.len() + 1 // guild_name: CString and Null Terminator
    }
}

impl MaximumPossibleSized for SMSG_GUILD_INVITE {
    fn maximum_possible_size() -> usize {
        256 // player_name: CString
        + 256 // guild_name: CString
    }
}

#[derive(Debug)]
pub enum SMSG_GUILD_INVITEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_GUILD_INVITEError {}
impl std::fmt::Display for SMSG_GUILD_INVITEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_GUILD_INVITEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_GUILD_INVITEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

