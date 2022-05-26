use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{ServerMessageType, ServerMessageTypeError};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_SERVER_MESSAGE {
    pub message_type: ServerMessageType,
    pub message: String,
}

impl SMSG_SERVER_MESSAGE {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // message_type: ServerMessageType
        w.write_all(&(self.message_type.as_int() as u32).to_le_bytes())?;

        // message: CString
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(w)
    }
}

impl ServerMessage for SMSG_SERVER_MESSAGE {
    const OPCODE: u16 = 0x0291;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_SERVER_MESSAGEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // message_type: ServerMessageType
        let message_type: ServerMessageType = crate::util::read_u32_le(r)?.try_into()?;

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
            // message_type: ServerMessageType
            let message_type: ServerMessageType = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            // message: CString
            let message = crate::util::tokio_read_c_string_to_vec(r).await?;
            let message = String::from_utf8(message)?;

            Ok(Self {
                message_type,
                message,
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
            // message_type: ServerMessageType
            let message_type: ServerMessageType = crate::util::astd_read_u32_le(r).await?.try_into()?;

            // message: CString
            let message = crate::util::astd_read_c_string_to_vec(r).await?;
            let message = String::from_utf8(message)?;

            Ok(Self {
                message_type,
                message,
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

impl SMSG_SERVER_MESSAGE {
    pub fn size(&self) -> usize {
        0
        + 4 // message_type: ServerMessageType
        + self.message.len() + 1 // message: CString
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

