use std::convert::{TryFrom, TryInto};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_GROUP_CHANGE_SUB_GROUP {
    pub name: String,
    pub group_number: u8,
}

impl ClientMessageWrite for CMSG_GROUP_CHANGE_SUB_GROUP {}

impl MessageBody for CMSG_GROUP_CHANGE_SUB_GROUP {
    const OPCODE: u16 = 0x027e;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_GROUP_CHANGE_SUB_GROUPError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // group_number: u8
        let group_number = crate::util::read_u8_le(r)?;

        Ok(Self {
            name,
            group_number,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // group_number: u8
        w.write_all(&self.group_number.to_le_bytes())?;

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
            // name: CString
            let name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let name = String::from_utf8(name)?;

            // group_number: u8
            let group_number = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                name,
                group_number,
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
            // name: CString
            w.write_all(self.name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // group_number: u8
            w.write_all(&self.group_number.to_le_bytes()).await?;

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
            // name: CString
            let name = crate::util::astd_read_c_string_to_vec(r).await?;
            let name = String::from_utf8(name)?;

            // group_number: u8
            let group_number = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                name,
                group_number,
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
            // name: CString
            w.write_all(self.name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // group_number: u8
            w.write_all(&self.group_number.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl CMSG_GROUP_CHANGE_SUB_GROUP {
    pub fn size(&self) -> usize {
        0
        + self.name.len() + 1 // name: CString
        + 1 // group_number: u8
    }
}

#[derive(Debug)]
pub enum CMSG_GROUP_CHANGE_SUB_GROUPError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_GROUP_CHANGE_SUB_GROUPError {}
impl std::fmt::Display for CMSG_GROUP_CHANGE_SUB_GROUPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_GROUP_CHANGE_SUB_GROUPError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_GROUP_CHANGE_SUB_GROUPError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

