use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Emote, EmoteError};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_EMOTE {
    pub emote: Emote,
}

impl CMSG_EMOTE {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 4], std::io::Error> {
        let mut array_w = [0u8; 4];
        let mut w = array_w.as_mut_slice();
        // emote: Emote
        w.write_all(&(self.emote.as_int() as u32).to_le_bytes())?;

        Ok(array_w)
    }
}

impl ClientMessage for CMSG_EMOTE {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(4);
        // emote: Emote
        w.write_all(&(self.emote.as_int() as u32).to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x0102;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    type Error = CMSG_EMOTEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // emote: Emote
        let emote: Emote = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            emote,
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
            // emote: Emote
            let emote: Emote = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            Ok(Self {
                emote,
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
            // emote: Emote
            let emote: Emote = crate::util::astd_read_u32_le(r).await?.try_into()?;

            Ok(Self {
                emote,
            })
        })
    }

}

#[derive(Debug)]
pub enum CMSG_EMOTEError {
    Io(std::io::Error),
    Emote(EmoteError),
}

impl std::error::Error for CMSG_EMOTEError {}
impl std::fmt::Display for CMSG_EMOTEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Emote(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_EMOTEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<EmoteError> for CMSG_EMOTEError {
    fn from(e: EmoteError) -> Self {
        Self::Emote(e)
    }
}

