use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_ADD_FRIEND {
    pub friend_name: String,
}

impl CMSG_ADD_FRIEND {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // friend_name: CString
        w.write_all(self.friend_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(w)
    }
}

impl ClientMessage for CMSG_ADD_FRIEND {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // friend_name: CString
        w.write_all(self.friend_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(w)
    }
    const OPCODE: u16 = 0x0069;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_ADD_FRIENDError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // friend_name: CString
        let friend_name = crate::util::read_c_string_to_vec(r)?;
        let friend_name = String::from_utf8(friend_name)?;

        Ok(Self {
            friend_name,
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
            // friend_name: CString
            let friend_name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let friend_name = String::from_utf8(friend_name)?;

            Ok(Self {
                friend_name,
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
            // friend_name: CString
            let friend_name = crate::util::astd_read_c_string_to_vec(r).await?;
            let friend_name = String::from_utf8(friend_name)?;

            Ok(Self {
                friend_name,
            })
        })
    }

}

impl CMSG_ADD_FRIEND {
    pub fn size(&self) -> usize {
        0
        + self.friend_name.len() + 1 // friend_name: CString
    }
}

#[derive(Debug)]
pub enum CMSG_ADD_FRIENDError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_ADD_FRIENDError {}
impl std::fmt::Display for CMSG_ADD_FRIENDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_ADD_FRIENDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_ADD_FRIENDError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

