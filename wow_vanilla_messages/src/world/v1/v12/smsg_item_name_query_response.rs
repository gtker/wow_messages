use std::convert::{TryFrom, TryInto};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_ITEM_NAME_QUERY_RESPONSE {
    pub item_id: u32,
    pub item_name: String,
}

impl ServerMessageWrite for SMSG_ITEM_NAME_QUERY_RESPONSE {}

impl MessageBody for SMSG_ITEM_NAME_QUERY_RESPONSE {
    const OPCODE: u16 = 0x02c5;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_ITEM_NAME_QUERY_RESPONSEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // item_name: CString
        let item_name = crate::util::read_c_string_to_vec(r)?;
        let item_name = String::from_utf8(item_name)?;

        Ok(Self {
            item_id,
            item_name,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_name: CString
        w.write_all(self.item_name.as_bytes())?;
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
            // item_id: u32
            let item_id = crate::util::tokio_read_u32_le(r).await?;

            // item_name: CString
            let item_name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let item_name = String::from_utf8(item_name)?;

            Ok(Self {
                item_id,
                item_name,
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
            // item_id: u32
            w.write_all(&self.item_id.to_le_bytes()).await?;

            // item_name: CString
            w.write_all(self.item_name.as_bytes()).await?;
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
            // item_id: u32
            let item_id = crate::util::astd_read_u32_le(r).await?;

            // item_name: CString
            let item_name = crate::util::astd_read_c_string_to_vec(r).await?;
            let item_name = String::from_utf8(item_name)?;

            Ok(Self {
                item_id,
                item_name,
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
            // item_id: u32
            w.write_all(&self.item_id.to_le_bytes()).await?;

            // item_name: CString
            w.write_all(self.item_name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            Ok(())
        })
    }

}

impl VariableSized for SMSG_ITEM_NAME_QUERY_RESPONSE {
    fn size(&self) -> usize {
        0
        + 4 // item_id: u32
        + self.item_name.len() + 1 // item_name: CString
    }
}

impl MaximumPossibleSized for SMSG_ITEM_NAME_QUERY_RESPONSE {
    fn maximum_possible_size() -> usize {
        0
        + 4 // item_id: u32
        + 256 // item_name: CString
    }
}

#[derive(Debug)]
pub enum SMSG_ITEM_NAME_QUERY_RESPONSEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_ITEM_NAME_QUERY_RESPONSEError {}
impl std::fmt::Display for SMSG_ITEM_NAME_QUERY_RESPONSEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_ITEM_NAME_QUERY_RESPONSEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_ITEM_NAME_QUERY_RESPONSEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

