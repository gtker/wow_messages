use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_RESURRECT_REQUEST {
    pub guid: Guid,
    pub name_length: u32,
    pub name: String,
    pub caster_is_spirit_healer: u8,
    pub respect_resurrection_timer: u8,
}

impl ServerMessageWrite for SMSG_RESURRECT_REQUEST {}

impl SMSG_RESURRECT_REQUEST {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8000);
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // name_length: u32
        w.write_all(&self.name_length.to_le_bytes())?;

        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // caster_is_spirit_healer: u8
        w.write_all(&self.caster_is_spirit_healer.to_le_bytes())?;

        // respect_resurrection_timer: u8
        w.write_all(&self.respect_resurrection_timer.to_le_bytes())?;

        Ok(w)
    }
}

impl MessageBody for SMSG_RESURRECT_REQUEST {
    const OPCODE: u16 = 0x015b;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_RESURRECT_REQUESTError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // name_length: u32
        let name_length = crate::util::read_u32_le(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // caster_is_spirit_healer: u8
        let caster_is_spirit_healer = crate::util::read_u8_le(r)?;

        // respect_resurrection_timer: u8
        let respect_resurrection_timer = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            name_length,
            name,
            caster_is_spirit_healer,
            respect_resurrection_timer,
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
            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            // name_length: u32
            let name_length = crate::util::tokio_read_u32_le(r).await?;

            // name: CString
            let name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let name = String::from_utf8(name)?;

            // caster_is_spirit_healer: u8
            let caster_is_spirit_healer = crate::util::tokio_read_u8_le(r).await?;

            // respect_resurrection_timer: u8
            let respect_resurrection_timer = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                guid,
                name_length,
                name,
                caster_is_spirit_healer,
                respect_resurrection_timer,
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
            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            // name_length: u32
            let name_length = crate::util::astd_read_u32_le(r).await?;

            // name: CString
            let name = crate::util::astd_read_c_string_to_vec(r).await?;
            let name = String::from_utf8(name)?;

            // caster_is_spirit_healer: u8
            let caster_is_spirit_healer = crate::util::astd_read_u8_le(r).await?;

            // respect_resurrection_timer: u8
            let respect_resurrection_timer = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                guid,
                name_length,
                name,
                caster_is_spirit_healer,
                respect_resurrection_timer,
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

impl SMSG_RESURRECT_REQUEST {
    pub fn size(&self) -> usize {
        0
        + 8 // guid: Guid
        + 4 // name_length: u32
        + self.name.len() + 1 // name: CString
        + 1 // caster_is_spirit_healer: u8
        + 1 // respect_resurrection_timer: u8
    }
}

#[derive(Debug)]
pub enum SMSG_RESURRECT_REQUESTError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_RESURRECT_REQUESTError {}
impl std::fmt::Display for SMSG_RESURRECT_REQUESTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_RESURRECT_REQUESTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_RESURRECT_REQUESTError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

