use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{RaidGroupError, RaidGroupErrorError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_RAID_GROUP_ONLY {
    pub homebind_timer: u32,
    pub error: RaidGroupError,
}

impl ServerMessageWrite for SMSG_RAID_GROUP_ONLY {}

impl SMSG_RAID_GROUP_ONLY {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 8], std::io::Error> {
        let mut array_w = [0u8; 8];
        let mut w = array_w.as_mut_slice();
        // homebind_timer: u32
        w.write_all(&self.homebind_timer.to_le_bytes())?;

        // error: RaidGroupError
        w.write_all(&(self.error.as_int() as u32).to_le_bytes())?;

        Ok(array_w)
    }
}

impl MessageBody for SMSG_RAID_GROUP_ONLY {
    const OPCODE: u16 = 0x0286;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    type Error = SMSG_RAID_GROUP_ONLYError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // homebind_timer: u32
        let homebind_timer = crate::util::read_u32_le(r)?;

        // error: RaidGroupError
        let error: RaidGroupError = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            homebind_timer,
            error,
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
            // homebind_timer: u32
            let homebind_timer = crate::util::tokio_read_u32_le(r).await?;

            // error: RaidGroupError
            let error: RaidGroupError = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            Ok(Self {
                homebind_timer,
                error,
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
            // homebind_timer: u32
            let homebind_timer = crate::util::astd_read_u32_le(r).await?;

            // error: RaidGroupError
            let error: RaidGroupError = crate::util::astd_read_u32_le(r).await?.try_into()?;

            Ok(Self {
                homebind_timer,
                error,
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

#[derive(Debug)]
pub enum SMSG_RAID_GROUP_ONLYError {
    Io(std::io::Error),
    RaidGroupError(RaidGroupErrorError),
}

impl std::error::Error for SMSG_RAID_GROUP_ONLYError {}
impl std::fmt::Display for SMSG_RAID_GROUP_ONLYError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::RaidGroupError(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_RAID_GROUP_ONLYError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<RaidGroupErrorError> for SMSG_RAID_GROUP_ONLYError {
    fn from(e: RaidGroupErrorError) -> Self {
        Self::RaidGroupError(e)
    }
}

