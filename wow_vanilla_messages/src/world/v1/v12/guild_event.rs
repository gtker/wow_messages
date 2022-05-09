use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GuildEvent {
    PROMOTION,
    DEMOTION,
    MOTD,
    JOINED,
    LEFT,
    REMOVED,
    LEADER_IS,
    LEADER_CHANGED,
    DISBANDED,
    TABARD_CHANGED,
    UNKNOWN10,
    ROSTER_UPDATE,
    SIGNED_ON,
    SIGNED_OFF,
}

impl ReadableAndWritable for GuildEvent {
    type Error = GuildEventError;

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

    #[cfg(feature = "async_tokio")]
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

    #[cfg(feature = "async_tokio")]
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

    #[cfg(feature = "async_std")]
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

    #[cfg(feature = "async_std")]
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

impl GuildEvent {
    #[cfg(feature = "sync")]
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GuildEventError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u16_le<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, GuildEventError> {
        let a = crate::util::tokio_read_u16_le(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u16_le<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, GuildEventError> {
        let a = crate::util::astd_read_u16_le(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GuildEventError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u16_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, GuildEventError> {
        let a = crate::util::tokio_read_u16_be(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u16_be<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, GuildEventError> {
        let a = crate::util::astd_read_u16_be(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GuildEventError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u32_le<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, GuildEventError> {
        let a = crate::util::tokio_read_u32_le(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u32_le<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, GuildEventError> {
        let a = crate::util::astd_read_u32_le(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GuildEventError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u32_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, GuildEventError> {
        let a = crate::util::tokio_read_u32_be(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u32_be<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, GuildEventError> {
        let a = crate::util::astd_read_u32_be(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GuildEventError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u64_le<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, GuildEventError> {
        let a = crate::util::tokio_read_u64_le(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u64_le<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, GuildEventError> {
        let a = crate::util::astd_read_u64_le(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "sync")]
    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GuildEventError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u64_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, GuildEventError> {
        let a = crate::util::tokio_read_u64_be(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u64_be<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, GuildEventError> {
        let a = crate::util::astd_read_u64_be(r).await?;
        Ok((a as u8).try_into()?)
    }

    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::PROMOTION => 0x0,
            Self::DEMOTION => 0x1,
            Self::MOTD => 0x2,
            Self::JOINED => 0x3,
            Self::LEFT => 0x4,
            Self::REMOVED => 0x5,
            Self::LEADER_IS => 0x6,
            Self::LEADER_CHANGED => 0x7,
            Self::DISBANDED => 0x8,
            Self::TABARD_CHANGED => 0x9,
            Self::UNKNOWN10 => 0xa,
            Self::ROSTER_UPDATE => 0xb,
            Self::SIGNED_ON => 0xc,
            Self::SIGNED_OFF => 0xd,
        }
    }

    pub const fn new() -> Self {
        Self::PROMOTION
    }

}

impl ConstantSized for GuildEvent {}

impl MaximumPossibleSized for GuildEvent {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for GuildEvent {
    fn default() -> Self {
        Self::PROMOTION
    }
}

impl std::fmt::Display for GuildEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PROMOTION => f.write_str("PROMOTION"),
            Self::DEMOTION => f.write_str("DEMOTION"),
            Self::MOTD => f.write_str("MOTD"),
            Self::JOINED => f.write_str("JOINED"),
            Self::LEFT => f.write_str("LEFT"),
            Self::REMOVED => f.write_str("REMOVED"),
            Self::LEADER_IS => f.write_str("LEADER_IS"),
            Self::LEADER_CHANGED => f.write_str("LEADER_CHANGED"),
            Self::DISBANDED => f.write_str("DISBANDED"),
            Self::TABARD_CHANGED => f.write_str("TABARD_CHANGED"),
            Self::UNKNOWN10 => f.write_str("UNKNOWN10"),
            Self::ROSTER_UPDATE => f.write_str("ROSTER_UPDATE"),
            Self::SIGNED_ON => f.write_str("SIGNED_ON"),
            Self::SIGNED_OFF => f.write_str("SIGNED_OFF"),
        }
    }
}

impl TryFrom<u8> for GuildEvent {
    type Error = TryFromGuildEventError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PROMOTION),
            1 => Ok(Self::DEMOTION),
            2 => Ok(Self::MOTD),
            3 => Ok(Self::JOINED),
            4 => Ok(Self::LEFT),
            5 => Ok(Self::REMOVED),
            6 => Ok(Self::LEADER_IS),
            7 => Ok(Self::LEADER_CHANGED),
            8 => Ok(Self::DISBANDED),
            9 => Ok(Self::TABARD_CHANGED),
            10 => Ok(Self::UNKNOWN10),
            11 => Ok(Self::ROSTER_UPDATE),
            12 => Ok(Self::SIGNED_ON),
            13 => Ok(Self::SIGNED_OFF),
            _ => Err(TryFromGuildEventError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromGuildEventError {
    value: u8,
}

impl TryFromGuildEventError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum GuildEventError {
    Read(std::io::Error),
    TryFrom(TryFromGuildEventError),
}

impl std::error::Error for GuildEventError {}
impl std::fmt::Display for TryFromGuildEventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'GuildEvent': '{}'", self.value))
    }
}

impl std::fmt::Display for GuildEventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for GuildEventError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromGuildEventError> for GuildEventError {
    fn from(value: TryFromGuildEventError) -> Self {
        Self::TryFrom(value)
    }
}

