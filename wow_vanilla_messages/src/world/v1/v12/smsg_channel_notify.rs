use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{ChatNotify, ChatNotifyError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_CHANNEL_NOTIFY {
    pub notify_type: ChatNotify,
    pub channel_name: String,
}

impl ServerMessageWrite for SMSG_CHANNEL_NOTIFY {}

impl MessageBody for SMSG_CHANNEL_NOTIFY {
    const OPCODE: u16 = 0x0099;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_CHANNEL_NOTIFYError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // notify_type: ChatNotify
        let notify_type: ChatNotify = crate::util::read_u8_le(r)?.try_into()?;

        // channel_name: CString
        let channel_name = crate::util::read_c_string_to_vec(r)?;
        let channel_name = String::from_utf8(channel_name)?;

        Ok(Self {
            notify_type,
            channel_name,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // notify_type: ChatNotify
        w.write_all(&(self.notify_type.as_int() as u8).to_le_bytes())?;

        // channel_name: CString
        w.write_all(self.channel_name.as_bytes())?;
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
            // notify_type: ChatNotify
            let notify_type: ChatNotify = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            // channel_name: CString
            let channel_name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let channel_name = String::from_utf8(channel_name)?;

            Ok(Self {
                notify_type,
                channel_name,
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
            // notify_type: ChatNotify
            w.write_all(&(self.notify_type.as_int() as u8).to_le_bytes()).await?;

            // channel_name: CString
            w.write_all(self.channel_name.as_bytes()).await?;
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
            // notify_type: ChatNotify
            let notify_type: ChatNotify = crate::util::astd_read_u8_le(r).await?.try_into()?;

            // channel_name: CString
            let channel_name = crate::util::astd_read_c_string_to_vec(r).await?;
            let channel_name = String::from_utf8(channel_name)?;

            Ok(Self {
                notify_type,
                channel_name,
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
            // notify_type: ChatNotify
            w.write_all(&(self.notify_type.as_int() as u8).to_le_bytes()).await?;

            // channel_name: CString
            w.write_all(self.channel_name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            Ok(())
        })
    }

}

impl VariableSized for SMSG_CHANNEL_NOTIFY {
    fn size(&self) -> usize {
        0
        + 1 // notify_type: ChatNotify
        + self.channel_name.len() + 1 // channel_name: CString
    }
}

impl MaximumPossibleSized for SMSG_CHANNEL_NOTIFY {
    fn maximum_possible_size() -> usize {
        0
        + 1 // notify_type: ChatNotify
        + 256 // channel_name: CString
    }
}

#[derive(Debug)]
pub enum SMSG_CHANNEL_NOTIFYError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    ChatNotify(ChatNotifyError),
}

impl std::error::Error for SMSG_CHANNEL_NOTIFYError {}
impl std::fmt::Display for SMSG_CHANNEL_NOTIFYError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::ChatNotify(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_CHANNEL_NOTIFYError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_CHANNEL_NOTIFYError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<ChatNotifyError> for SMSG_CHANNEL_NOTIFYError {
    fn from(e: ChatNotifyError) -> Self {
        Self::ChatNotify(e)
    }
}

