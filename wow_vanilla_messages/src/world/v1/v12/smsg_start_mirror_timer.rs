use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{TimerType, TimerTypeError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_START_MIRROR_TIMER {
    pub timer: TimerType,
    pub time_remaining: u32,
    pub duration: u32,
    pub scale: u32,
    pub is_frozen: u8,
    pub id: u32,
}

impl ServerMessageWrite for SMSG_START_MIRROR_TIMER {}

impl MessageBody for SMSG_START_MIRROR_TIMER {
    const OPCODE: u16 = 0x01d9;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_START_MIRROR_TIMERError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // timer: TimerType
        let timer: TimerType = crate::util::read_u32_le(r)?.try_into()?;

        // time_remaining: u32
        let time_remaining = crate::util::read_u32_le(r)?;

        // duration: u32
        let duration = crate::util::read_u32_le(r)?;

        // scale: u32
        let scale = crate::util::read_u32_le(r)?;

        // is_frozen: u8
        let is_frozen = crate::util::read_u8_le(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        Ok(Self {
            timer,
            time_remaining,
            duration,
            scale,
            is_frozen,
            id,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // timer: TimerType
        w.write_all(&(self.timer.as_int() as u32).to_le_bytes())?;

        // time_remaining: u32
        w.write_all(&self.time_remaining.to_le_bytes())?;

        // duration: u32
        w.write_all(&self.duration.to_le_bytes())?;

        // scale: u32
        w.write_all(&self.scale.to_le_bytes())?;

        // is_frozen: u8
        w.write_all(&self.is_frozen.to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        Ok(())
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
            // timer: TimerType
            let timer: TimerType = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            // time_remaining: u32
            let time_remaining = crate::util::tokio_read_u32_le(r).await?;

            // duration: u32
            let duration = crate::util::tokio_read_u32_le(r).await?;

            // scale: u32
            let scale = crate::util::tokio_read_u32_le(r).await?;

            // is_frozen: u8
            let is_frozen = crate::util::tokio_read_u8_le(r).await?;

            // id: u32
            let id = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                timer,
                time_remaining,
                duration,
                scale,
                is_frozen,
                id,
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
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // timer: TimerType
            w.write_all(&(self.timer.as_int() as u32).to_le_bytes()).await?;

            // time_remaining: u32
            w.write_all(&self.time_remaining.to_le_bytes()).await?;

            // duration: u32
            w.write_all(&self.duration.to_le_bytes()).await?;

            // scale: u32
            w.write_all(&self.scale.to_le_bytes()).await?;

            // is_frozen: u8
            w.write_all(&self.is_frozen.to_le_bytes()).await?;

            // id: u32
            w.write_all(&self.id.to_le_bytes()).await?;

            Ok(())
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
            // timer: TimerType
            let timer: TimerType = crate::util::astd_read_u32_le(r).await?.try_into()?;

            // time_remaining: u32
            let time_remaining = crate::util::astd_read_u32_le(r).await?;

            // duration: u32
            let duration = crate::util::astd_read_u32_le(r).await?;

            // scale: u32
            let scale = crate::util::astd_read_u32_le(r).await?;

            // is_frozen: u8
            let is_frozen = crate::util::astd_read_u8_le(r).await?;

            // id: u32
            let id = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                timer,
                time_remaining,
                duration,
                scale,
                is_frozen,
                id,
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
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // timer: TimerType
            w.write_all(&(self.timer.as_int() as u32).to_le_bytes()).await?;

            // time_remaining: u32
            w.write_all(&self.time_remaining.to_le_bytes()).await?;

            // duration: u32
            w.write_all(&self.duration.to_le_bytes()).await?;

            // scale: u32
            w.write_all(&self.scale.to_le_bytes()).await?;

            // is_frozen: u8
            w.write_all(&self.is_frozen.to_le_bytes()).await?;

            // id: u32
            w.write_all(&self.id.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl SMSG_START_MIRROR_TIMER {
    pub(crate) fn size() -> usize {
        0
        + 4 // timer: TimerType
        + 4 // time_remaining: u32
        + 4 // duration: u32
        + 4 // scale: u32
        + 1 // is_frozen: u8
        + 4 // id: u32
    }
}

#[derive(Debug)]
pub enum SMSG_START_MIRROR_TIMERError {
    Io(std::io::Error),
    TimerType(TimerTypeError),
}

impl std::error::Error for SMSG_START_MIRROR_TIMERError {}
impl std::fmt::Display for SMSG_START_MIRROR_TIMERError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::TimerType(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_START_MIRROR_TIMERError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<TimerTypeError> for SMSG_START_MIRROR_TIMERError {
    fn from(e: TimerTypeError) -> Self {
        Self::TimerType(e)
    }
}

