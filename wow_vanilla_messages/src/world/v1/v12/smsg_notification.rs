use std::convert::{TryFrom, TryInto};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_NOTIFICATION {
    pub notification: String,
}

impl ServerMessageWrite for SMSG_NOTIFICATION {}

impl MessageBody for SMSG_NOTIFICATION {
    const OPCODE: u16 = 0x01cb;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_NOTIFICATIONError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // notification: CString
        let notification = crate::util::read_c_string_to_vec(r)?;
        let notification = String::from_utf8(notification)?;

        Ok(Self {
            notification,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // notification: CString
        w.write_all(self.notification.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
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
            // notification: CString
            let notification = crate::util::tokio_read_c_string_to_vec(r).await?;
            let notification = String::from_utf8(notification)?;

            Ok(Self {
                notification,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // notification: CString
            w.write_all(self.notification.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
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
            // notification: CString
            let notification = crate::util::astd_read_c_string_to_vec(r).await?;
            let notification = String::from_utf8(notification)?;

            Ok(Self {
                notification,
            })
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // notification: CString
            w.write_all(self.notification.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            Ok(())
        })
    }

}

impl VariableSized for SMSG_NOTIFICATION {
    fn size(&self) -> usize {
        0
        + self.notification.len() + 1 // notification: CString
    }
}

impl MaximumPossibleSized for SMSG_NOTIFICATION {
    fn maximum_possible_size() -> usize {
        0
        + 256 // notification: CString
    }
}

#[derive(Debug)]
pub enum SMSG_NOTIFICATIONError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_NOTIFICATIONError {}
impl std::fmt::Display for SMSG_NOTIFICATIONError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_NOTIFICATIONError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_NOTIFICATIONError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

