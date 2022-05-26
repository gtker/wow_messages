use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{UnitStandState, UnitStandStateError};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_STANDSTATECHANGE {
    pub animation_state: UnitStandState,
}

impl CMSG_STANDSTATECHANGE {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 1], std::io::Error> {
        let mut array_w = [0u8; 1];
        let mut w = array_w.as_mut_slice();
        // animation_state: UnitStandState
        w.write_all(&(self.animation_state.as_int() as u32).to_le_bytes())?;

        Ok(array_w)
    }
}

impl ClientMessage for CMSG_STANDSTATECHANGE {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(1);
        // animation_state: UnitStandState
        w.write_all(&(self.animation_state.as_int() as u32).to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x0101;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        1
    }

    type Error = CMSG_STANDSTATECHANGEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // animation_state: UnitStandState
        let animation_state: UnitStandState = (crate::util::read_u32_le(r)? as u8).try_into()?;

        Ok(Self {
            animation_state,
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
            // animation_state: UnitStandState
            let animation_state: UnitStandState = (crate::util::tokio_read_u32_le(r).await? as u8).try_into()?;

            Ok(Self {
                animation_state,
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
            // animation_state: UnitStandState
            let animation_state: UnitStandState = (crate::util::astd_read_u32_le(r).await? as u8).try_into()?;

            Ok(Self {
                animation_state,
            })
        })
    }

}

#[derive(Debug)]
pub enum CMSG_STANDSTATECHANGEError {
    Io(std::io::Error),
    UnitStandState(UnitStandStateError),
}

impl std::error::Error for CMSG_STANDSTATECHANGEError {}
impl std::fmt::Display for CMSG_STANDSTATECHANGEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::UnitStandState(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_STANDSTATECHANGEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<UnitStandStateError> for CMSG_STANDSTATECHANGEError {
    fn from(e: UnitStandStateError) -> Self {
        Self::UnitStandState(e)
    }
}

