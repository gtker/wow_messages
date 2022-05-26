use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Map, MapError};
use crate::world::v1::v12::{TransferAbortReason, TransferAbortReasonError};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_TRANSFER_ABORTED {
    pub map: Map,
    pub reason: TransferAbortReason,
}

impl SMSG_TRANSFER_ABORTED {
    pub const PADDING_VALUE: u8 = 0x00;

}

impl SMSG_TRANSFER_ABORTED {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 6], std::io::Error> {
        let mut array_w = [0u8; 6];
        let mut w = array_w.as_mut_slice();
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // reason: TransferAbortReason
        w.write_all(&(self.reason.as_int() as u8).to_le_bytes())?;

        // padding: u8
        w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_TRANSFER_ABORTED {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(6);
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // reason: TransferAbortReason
        w.write_all(&(self.reason.as_int() as u8).to_le_bytes())?;

        // padding: u8
        w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x0040;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        6
    }

    type Error = SMSG_TRANSFER_ABORTEDError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // reason: TransferAbortReason
        let reason: TransferAbortReason = crate::util::read_u8_le(r)?.try_into()?;

        // padding: u8
        let _padding = crate::util::read_u8_le(r)?;
        // padding is expected to always be 0 (0)

        Ok(Self {
            map,
            reason,
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
            // map: Map
            let map: Map = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            // reason: TransferAbortReason
            let reason: TransferAbortReason = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            // padding: u8
            let _padding = crate::util::tokio_read_u8_le(r).await?;
            // padding is expected to always be 0 (0)

            Ok(Self {
                map,
                reason,
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
            // map: Map
            let map: Map = crate::util::astd_read_u32_le(r).await?.try_into()?;

            // reason: TransferAbortReason
            let reason: TransferAbortReason = crate::util::astd_read_u8_le(r).await?.try_into()?;

            // padding: u8
            let _padding = crate::util::astd_read_u8_le(r).await?;
            // padding is expected to always be 0 (0)

            Ok(Self {
                map,
                reason,
            })
        })
    }

}

#[derive(Debug)]
pub enum SMSG_TRANSFER_ABORTEDError {
    Io(std::io::Error),
    Map(MapError),
    TransferAbortReason(TransferAbortReasonError),
}

impl std::error::Error for SMSG_TRANSFER_ABORTEDError {}
impl std::fmt::Display for SMSG_TRANSFER_ABORTEDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
            Self::TransferAbortReason(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_TRANSFER_ABORTEDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MapError> for SMSG_TRANSFER_ABORTEDError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

impl From<TransferAbortReasonError> for SMSG_TRANSFER_ABORTEDError {
    fn from(e: TransferAbortReasonError) -> Self {
        Self::TransferAbortReason(e)
    }
}

