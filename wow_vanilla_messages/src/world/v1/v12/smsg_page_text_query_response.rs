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
pub struct SMSG_PAGE_TEXT_QUERY_RESPONSE {
    pub page_id: u32,
    pub text: String,
    pub next_page_id: u32,
}

impl ServerMessageWrite for SMSG_PAGE_TEXT_QUERY_RESPONSE {}

impl MessageBody for SMSG_PAGE_TEXT_QUERY_RESPONSE {
    const OPCODE: u16 = 0x005b;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_PAGE_TEXT_QUERY_RESPONSEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // page_id: u32
        let page_id = crate::util::read_u32_le(r)?;

        // text: CString
        let text = crate::util::read_c_string_to_vec(r)?;
        let text = String::from_utf8(text)?;

        // next_page_id: u32
        let next_page_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            page_id,
            text,
            next_page_id,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // page_id: u32
        w.write_all(&self.page_id.to_le_bytes())?;

        // text: CString
        w.write_all(self.text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // next_page_id: u32
        w.write_all(&self.next_page_id.to_le_bytes())?;

        Ok(())
    }

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
            // page_id: u32
            let page_id = crate::util::tokio_read_u32_le(r).await?;

            // text: CString
            let text = crate::util::tokio_read_c_string_to_vec(r).await?;
            let text = String::from_utf8(text)?;

            // next_page_id: u32
            let next_page_id = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                page_id,
                text,
                next_page_id,
            })
        })
    }

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
            // page_id: u32
            w.write_all(&self.page_id.to_le_bytes()).await?;

            // text: CString
            w.write_all(self.text.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // next_page_id: u32
            w.write_all(&self.next_page_id.to_le_bytes()).await?;

            Ok(())
        })
    }

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
            // page_id: u32
            let page_id = crate::util::astd_read_u32_le(r).await?;

            // text: CString
            let text = crate::util::astd_read_c_string_to_vec(r).await?;
            let text = String::from_utf8(text)?;

            // next_page_id: u32
            let next_page_id = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                page_id,
                text,
                next_page_id,
            })
        })
    }

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
            // page_id: u32
            w.write_all(&self.page_id.to_le_bytes()).await?;

            // text: CString
            w.write_all(self.text.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // next_page_id: u32
            w.write_all(&self.next_page_id.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl VariableSized for SMSG_PAGE_TEXT_QUERY_RESPONSE {
    fn size(&self) -> usize {
        0
        + 4 // page_id: u32
        + self.text.len() + 1 // text: CString
        + 4 // next_page_id: u32
    }
}

impl MaximumPossibleSized for SMSG_PAGE_TEXT_QUERY_RESPONSE {
    fn maximum_possible_size() -> usize {
        0
        + 4 // page_id: u32
        + 256 // text: CString
        + 4 // next_page_id: u32
    }
}

#[derive(Debug)]
pub enum SMSG_PAGE_TEXT_QUERY_RESPONSEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_PAGE_TEXT_QUERY_RESPONSEError {}
impl std::fmt::Display for SMSG_PAGE_TEXT_QUERY_RESPONSEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PAGE_TEXT_QUERY_RESPONSEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_PAGE_TEXT_QUERY_RESPONSEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

