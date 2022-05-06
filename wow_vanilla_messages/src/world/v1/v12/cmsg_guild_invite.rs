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
pub struct CMSG_GUILD_INVITE {
    pub invited_player: String,
}

impl ClientMessageWrite for CMSG_GUILD_INVITE {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_GUILD_INVITE {
    const OPCODE: u16 = 0x0082;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_GUILD_INVITEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // invited_player: CString
        let invited_player = crate::util::read_c_string_to_vec(r)?;
        let invited_player = String::from_utf8(invited_player)?;

        Ok(Self {
            invited_player,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // invited_player: CString
        w.write_all(self.invited_player.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // invited_player: CString
        let invited_player = crate::util::tokio_read_c_string_to_vec(r).await?;
        let invited_player = String::from_utf8(invited_player)?;

        Ok(Self {
            invited_player,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // invited_player: CString
        w.write_all(self.invited_player.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // invited_player: CString
        let invited_player = crate::util::astd_read_c_string_to_vec(r).await?;
        let invited_player = String::from_utf8(invited_player)?;

        Ok(Self {
            invited_player,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // invited_player: CString
        w.write_all(self.invited_player.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        Ok(())
    }

}

impl VariableSized for CMSG_GUILD_INVITE {
    fn size(&self) -> usize {
        0
        + self.invited_player.len() + 1 // invited_player: CString
    }
}

impl MaximumPossibleSized for CMSG_GUILD_INVITE {
    fn maximum_possible_size() -> usize {
        0
        + 256 // invited_player: CString
    }
}

#[derive(Debug)]
pub enum CMSG_GUILD_INVITEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_GUILD_INVITEError {}
impl std::fmt::Display for CMSG_GUILD_INVITEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_GUILD_INVITEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_GUILD_INVITEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

