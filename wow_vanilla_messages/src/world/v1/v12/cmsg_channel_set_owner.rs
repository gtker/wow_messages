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
pub struct CMSG_CHANNEL_SET_OWNER {
    pub channel_name: String,
    pub new_owner: String,
}

impl ClientMessageWrite for CMSG_CHANNEL_SET_OWNER {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_CHANNEL_SET_OWNER {
    const OPCODE: u16 = 0x009d;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_CHANNEL_SET_OWNERError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // channel_name: CString
        let channel_name = crate::util::read_c_string_to_vec(r)?;
        let channel_name = String::from_utf8(channel_name)?;

        // new_owner: CString
        let new_owner = crate::util::read_c_string_to_vec(r)?;
        let new_owner = String::from_utf8(new_owner)?;

        Ok(Self {
            channel_name,
            new_owner,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // channel_name: CString
        w.write_all(self.channel_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // new_owner: CString
        w.write_all(self.new_owner.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // channel_name: CString
        let channel_name = crate::util::tokio_read_c_string_to_vec(r).await?;
        let channel_name = String::from_utf8(channel_name)?;

        // new_owner: CString
        let new_owner = crate::util::tokio_read_c_string_to_vec(r).await?;
        let new_owner = String::from_utf8(new_owner)?;

        Ok(Self {
            channel_name,
            new_owner,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // channel_name: CString
        w.write_all(self.channel_name.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        // new_owner: CString
        w.write_all(self.new_owner.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // channel_name: CString
        let channel_name = crate::util::astd_read_c_string_to_vec(r).await?;
        let channel_name = String::from_utf8(channel_name)?;

        // new_owner: CString
        let new_owner = crate::util::astd_read_c_string_to_vec(r).await?;
        let new_owner = String::from_utf8(new_owner)?;

        Ok(Self {
            channel_name,
            new_owner,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // channel_name: CString
        w.write_all(self.channel_name.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        // new_owner: CString
        w.write_all(self.new_owner.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        Ok(())
    }

}

impl VariableSized for CMSG_CHANNEL_SET_OWNER {
    fn size(&self) -> usize {
        self.channel_name.len() + 1 // channel_name: CString and Null Terminator
        + self.new_owner.len() + 1 // new_owner: CString and Null Terminator
    }
}

impl MaximumPossibleSized for CMSG_CHANNEL_SET_OWNER {
    fn maximum_possible_size() -> usize {
        256 // channel_name: CString
        + 256 // new_owner: CString
    }
}

#[derive(Debug)]
pub enum CMSG_CHANNEL_SET_OWNERError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_CHANNEL_SET_OWNERError {}
impl std::fmt::Display for CMSG_CHANNEL_SET_OWNERError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_CHANNEL_SET_OWNERError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_CHANNEL_SET_OWNERError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

