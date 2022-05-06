use std::convert::{TryFrom, TryInto};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_GUILD_INFO_TEXT {
    pub guild_info: String,
}

impl ClientMessageWrite for CMSG_GUILD_INFO_TEXT {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_GUILD_INFO_TEXT {
    const OPCODE: u16 = 0x02fc;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_GUILD_INFO_TEXTError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guild_info: CString
        let guild_info = crate::util::read_c_string_to_vec(r)?;
        let guild_info = String::from_utf8(guild_info)?;

        Ok(Self {
            guild_info,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guild_info: CString
        w.write_all(self.guild_info.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guild_info: CString
        let guild_info = crate::util::tokio_read_c_string_to_vec(r).await?;
        let guild_info = String::from_utf8(guild_info)?;

        Ok(Self {
            guild_info,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guild_info: CString
        w.write_all(self.guild_info.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guild_info: CString
        let guild_info = crate::util::astd_read_c_string_to_vec(r).await?;
        let guild_info = String::from_utf8(guild_info)?;

        Ok(Self {
            guild_info,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guild_info: CString
        w.write_all(self.guild_info.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        Ok(())
    }

}

impl VariableSized for CMSG_GUILD_INFO_TEXT {
    fn size(&self) -> usize {
        0
        + self.guild_info.len() + 1 // guild_info: CString
    }
}

impl MaximumPossibleSized for CMSG_GUILD_INFO_TEXT {
    fn maximum_possible_size() -> usize {
        0
        + 256 // guild_info: CString
    }
}

#[derive(Debug)]
pub enum CMSG_GUILD_INFO_TEXTError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_GUILD_INFO_TEXTError {}
impl std::fmt::Display for CMSG_GUILD_INFO_TEXTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_GUILD_INFO_TEXTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_GUILD_INFO_TEXTError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

