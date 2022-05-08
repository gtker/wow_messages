use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum InstanceResetFailedReason {
    GENERAL,
    OFFLINE,
    ZONING,
    SILENTLY,
}

impl ReadableAndWritable for InstanceResetFailedReason {
    type Error = InstanceResetFailedReasonError;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_int().to_le_bytes())?;
        Ok(())
    }

    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let a = crate::util::tokio_read_u8_le(r).await?;

            Ok(a.try_into()?)
        })
    }

    fn tokio_write<'life0, 'life1, 'async_trait, W>(
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
            w.write_all(&self.as_int().to_le_bytes()).await?;
            Ok(())
        })
    }

    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let a = crate::util::astd_read_u8_le(r).await?;

            Ok(a.try_into()?)
        })
    }

    fn astd_write<'life0, 'life1, 'async_trait, W>(
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
            w.write_all(&self.as_int().to_le_bytes()).await?;
            Ok(())
        })
    }

}

impl InstanceResetFailedReason {
    #[cfg(feature = "sync")]
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, InstanceResetFailedReasonError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u16_le<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, InstanceResetFailedReasonError> {
        let a = crate::util::tokio_read_u16_le(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u16_le<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, InstanceResetFailedReasonError> {
        let a = crate::util::astd_read_u16_le(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_int() as u16)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u16_le(w, self.as_int() as u16).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u16_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::astd_write_u16_le(w, self.as_int() as u16).await?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, InstanceResetFailedReasonError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u16_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, InstanceResetFailedReasonError> {
        let a = crate::util::tokio_read_u16_be(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u16_be<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, InstanceResetFailedReasonError> {
        let a = crate::util::astd_read_u16_be(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_int() as u16)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u16_be(w, self.as_int() as u16).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u16_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::astd_write_u16_be(w, self.as_int() as u16).await?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, InstanceResetFailedReasonError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u32_le<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, InstanceResetFailedReasonError> {
        let a = crate::util::tokio_read_u32_le(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u32_le<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, InstanceResetFailedReasonError> {
        let a = crate::util::astd_read_u32_le(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_int() as u32)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u32_le(w, self.as_int() as u32).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::astd_write_u32_le(w, self.as_int() as u32).await?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, InstanceResetFailedReasonError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u32_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, InstanceResetFailedReasonError> {
        let a = crate::util::tokio_read_u32_be(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u32_be<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, InstanceResetFailedReasonError> {
        let a = crate::util::astd_read_u32_be(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_int() as u32)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u32_be(w, self.as_int() as u32).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::astd_write_u32_be(w, self.as_int() as u32).await?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, InstanceResetFailedReasonError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u64_le<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, InstanceResetFailedReasonError> {
        let a = crate::util::tokio_read_u64_le(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u64_le<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, InstanceResetFailedReasonError> {
        let a = crate::util::astd_read_u64_le(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_int() as u64)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u64_le(w, self.as_int() as u64).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::astd_write_u64_le(w, self.as_int() as u64).await?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, InstanceResetFailedReasonError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u64_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, InstanceResetFailedReasonError> {
        let a = crate::util::tokio_read_u64_be(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u64_be<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, InstanceResetFailedReasonError> {
        let a = crate::util::astd_read_u64_be(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_int() as u64)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u64_be(w, self.as_int() as u64).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::astd_write_u64_be(w, self.as_int() as u64).await?;
        Ok(())
    }

    pub const fn as_int(&self) -> u8 {
        match self {
            Self::GENERAL => 0x0,
            Self::OFFLINE => 0x1,
            Self::ZONING => 0x2,
            Self::SILENTLY => 0x3,
        }
    }

    pub const fn new() -> Self {
        Self::GENERAL
    }

}

impl ConstantSized for InstanceResetFailedReason {}

impl MaximumPossibleSized for InstanceResetFailedReason {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for InstanceResetFailedReason {
    fn default() -> Self {
        Self::GENERAL
    }
}

impl std::fmt::Display for InstanceResetFailedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GENERAL => f.write_str("GENERAL"),
            Self::OFFLINE => f.write_str("OFFLINE"),
            Self::ZONING => f.write_str("ZONING"),
            Self::SILENTLY => f.write_str("SILENTLY"),
        }
    }
}

impl TryFrom<u8> for InstanceResetFailedReason {
    type Error = TryFromInstanceResetFailedReasonError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::GENERAL),
            1 => Ok(Self::OFFLINE),
            2 => Ok(Self::ZONING),
            3 => Ok(Self::SILENTLY),
            _ => Err(TryFromInstanceResetFailedReasonError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromInstanceResetFailedReasonError {
    value: u8,
}

impl TryFromInstanceResetFailedReasonError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum InstanceResetFailedReasonError {
    Read(std::io::Error),
    TryFrom(TryFromInstanceResetFailedReasonError),
}

impl std::error::Error for InstanceResetFailedReasonError {}
impl std::fmt::Display for TryFromInstanceResetFailedReasonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'InstanceResetFailedReason': '{}'", self.value))
    }
}

impl std::fmt::Display for InstanceResetFailedReasonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for InstanceResetFailedReasonError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromInstanceResetFailedReasonError> for InstanceResetFailedReasonError {
    fn from(value: TryFromInstanceResetFailedReasonError) -> Self {
        Self::TryFrom(value)
    }
}

