use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CMSG_CHANNEL_PASSWORD {
    pub channel_name: String,
    pub channel_password: String,
}

impl CMSG_CHANNEL_PASSWORD {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // channel_name: CString
        w.write_all(self.channel_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // channel_password: CString
        w.write_all(self.channel_password.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(w)
    }
}

impl ClientMessage for CMSG_CHANNEL_PASSWORD {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // channel_name: CString
        w.write_all(self.channel_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // channel_password: CString
        w.write_all(self.channel_password.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x009c;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = CMSG_CHANNEL_PASSWORDError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // channel_name: CString
        let channel_name = crate::util::read_c_string_to_vec(r)?;
        let channel_name = String::from_utf8(channel_name)?;

        // channel_password: CString
        let channel_password = crate::util::read_c_string_to_vec(r)?;
        let channel_password = String::from_utf8(channel_password)?;

        Ok(Self {
            channel_name,
            channel_password,
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
            // channel_name: CString
            let channel_name = crate::util::tokio_read_c_string_to_vec(r).await?;
            let channel_name = String::from_utf8(channel_name)?;

            // channel_password: CString
            let channel_password = crate::util::tokio_read_c_string_to_vec(r).await?;
            let channel_password = String::from_utf8(channel_password)?;

            Ok(Self {
                channel_name,
                channel_password,
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
            // channel_name: CString
            let channel_name = crate::util::astd_read_c_string_to_vec(r).await?;
            let channel_name = String::from_utf8(channel_name)?;

            // channel_password: CString
            let channel_password = crate::util::astd_read_c_string_to_vec(r).await?;
            let channel_password = String::from_utf8(channel_password)?;

            Ok(Self {
                channel_name,
                channel_password,
            })
        })
    }

}

impl CMSG_CHANNEL_PASSWORD {
    pub fn size(&self) -> usize {
        0
        + self.channel_name.len() + 1 // channel_name: CString
        + self.channel_password.len() + 1 // channel_password: CString
    }
}

#[derive(Debug)]
pub enum CMSG_CHANNEL_PASSWORDError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_CHANNEL_PASSWORDError {}
impl std::fmt::Display for CMSG_CHANNEL_PASSWORDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_CHANNEL_PASSWORDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_CHANNEL_PASSWORDError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

