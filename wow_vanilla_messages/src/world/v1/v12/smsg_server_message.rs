use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{ServerMessageType, ServerMessageTypeError};
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
pub struct SMSG_SERVER_MESSAGE {
    pub message_type: ServerMessageType,
    pub message: String,
}

impl ServerMessageWrite for SMSG_SERVER_MESSAGE {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_SERVER_MESSAGE {
    const OPCODE: u16 = 0x0291;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_SERVER_MESSAGEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // message_type: ServerMessageType
        let message_type = ServerMessageType::read(r)?;

        // message: CString
        let message = crate::util::read_c_string_to_vec(r)?;
        let message = String::from_utf8(message)?;

        Ok(Self {
            message_type,
            message,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // message_type: ServerMessageType
        self.message_type.write(w)?;

        // message: CString
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // message_type: ServerMessageType
        let message_type = ServerMessageType::tokio_read(r).await?;

        // message: CString
        let message = crate::util::tokio_read_c_string_to_vec(r).await?;
        let message = String::from_utf8(message)?;

        Ok(Self {
            message_type,
            message,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // message_type: ServerMessageType
        self.message_type.tokio_write(w).await?;

        // message: CString
        w.write_all(self.message.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // message_type: ServerMessageType
        let message_type = ServerMessageType::astd_read(r).await?;

        // message: CString
        let message = crate::util::astd_read_c_string_to_vec(r).await?;
        let message = String::from_utf8(message)?;

        Ok(Self {
            message_type,
            message,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // message_type: ServerMessageType
        self.message_type.astd_write(w).await?;

        // message: CString
        w.write_all(self.message.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        Ok(())
    }

}

impl VariableSized for SMSG_SERVER_MESSAGE {
    fn size(&self) -> usize {
        ServerMessageType::size() // message_type: ServerMessageType
        + self.message.len() + 1 // message: CString and Null Terminator
    }
}

impl MaximumPossibleSized for SMSG_SERVER_MESSAGE {
    fn maximum_possible_size() -> usize {
        ServerMessageType::maximum_possible_size() // message_type: ServerMessageType
        + 256 // message: CString
    }
}

#[derive(Debug)]
pub enum SMSG_SERVER_MESSAGEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    ServerMessageType(ServerMessageTypeError),
}

impl std::error::Error for SMSG_SERVER_MESSAGEError {}
impl std::fmt::Display for SMSG_SERVER_MESSAGEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::ServerMessageType(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_SERVER_MESSAGEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_SERVER_MESSAGEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<ServerMessageTypeError> for SMSG_SERVER_MESSAGEError {
    fn from(e: ServerMessageTypeError) -> Self {
        Self::ServerMessageType(e)
    }
}

