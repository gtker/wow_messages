use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_PET_RENAME {
    pub pet_guid: Guid,
    pub name: String,
}

impl CMSG_PET_RENAME {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // pet_guid: Guid
        w.write_all(&self.pet_guid.guid().to_le_bytes())?;

        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(w)
    }
}

impl ClientMessage for CMSG_PET_RENAME {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // pet_guid: Guid
        w.write_all(&self.pet_guid.guid().to_le_bytes())?;

        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(w)
    }
    const OPCODE: u16 = 0x0177;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_PET_RENAMEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // pet_guid: Guid
        let pet_guid = Guid::read(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        Ok(Self {
            pet_guid,
            name,
        })
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
            // pet_guid: Guid
            let pet_guid = Guid::tokio_read(r).await?;

            // name: CString
            let name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let name = String::from_utf8(name)?;

            Ok(Self {
                pet_guid,
                name,
            })
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
            // pet_guid: Guid
            let pet_guid = Guid::astd_read(r).await?;

            // name: CString
            let name = crate::util::astd_read_c_string_to_vec(r).await?;
            let name = String::from_utf8(name)?;

            Ok(Self {
                pet_guid,
                name,
            })
        })
    }

}

impl CMSG_PET_RENAME {
    pub fn size(&self) -> usize {
        0
        + 8 // pet_guid: Guid
        + self.name.len() + 1 // name: CString
    }
}

#[derive(Debug)]
pub enum CMSG_PET_RENAMEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_PET_RENAMEError {}
impl std::fmt::Display for CMSG_PET_RENAMEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_PET_RENAMEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_PET_RENAMEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

