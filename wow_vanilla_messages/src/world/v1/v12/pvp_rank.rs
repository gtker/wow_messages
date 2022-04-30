use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PvpRank {
    NO_RANK,
    PARIAH,
    OUTLAW,
    EXILED,
    DISHONORED,
    RANK1,
    RANK2,
    RANK3,
    RANK4,
    RANK5,
    RANK6,
    RANK7,
    RANK8,
    RANK9,
    RANK10,
    RANK11,
    RANK12,
    RANK13,
    RANK14,
    FACTION_LEADER,
}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl ReadableAndWritable for PvpRank {
    type Error = PvpRankError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::tokio_read_u8_le(r).await?;

        Ok(a.try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes()).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::astd_read_u8_le(r).await?;

        Ok(a.try_into()?)
    }

    #[cfg(feature = "async_std")]
    async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes()).await?;
        Ok(())
    }

}

impl PvpRank {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PvpRankError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u16_le<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, PvpRankError> {
        let a = crate::util::tokio_read_u16_le(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u16_le<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, PvpRankError> {
        let a = crate::util::astd_read_u16_le(r).await?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u16_le(w, self.as_u8() as u16).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u16_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::astd_write_u16_le(w, self.as_u8() as u16).await?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PvpRankError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u16_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, PvpRankError> {
        let a = crate::util::tokio_read_u16_be(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u16_be<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, PvpRankError> {
        let a = crate::util::astd_read_u16_be(r).await?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u16_be(w, self.as_u8() as u16).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u16_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::astd_write_u16_be(w, self.as_u8() as u16).await?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PvpRankError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u32_le<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, PvpRankError> {
        let a = crate::util::tokio_read_u32_le(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u32_le<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, PvpRankError> {
        let a = crate::util::astd_read_u32_le(r).await?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u32_le(w, self.as_u8() as u32).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::astd_write_u32_le(w, self.as_u8() as u32).await?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PvpRankError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u32_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, PvpRankError> {
        let a = crate::util::tokio_read_u32_be(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u32_be<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, PvpRankError> {
        let a = crate::util::astd_read_u32_be(r).await?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u32_be(w, self.as_u8() as u32).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::astd_write_u32_be(w, self.as_u8() as u32).await?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PvpRankError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u64_le<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, PvpRankError> {
        let a = crate::util::tokio_read_u64_le(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u64_le<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, PvpRankError> {
        let a = crate::util::astd_read_u64_le(r).await?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u64_le(w, self.as_u8() as u64).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::astd_write_u64_le(w, self.as_u8() as u64).await?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PvpRankError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_read_u64_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, PvpRankError> {
        let a = crate::util::tokio_read_u64_be(r).await?;
        Ok((a as u8).try_into()?)
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_read_u64_be<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, PvpRankError> {
        let a = crate::util::astd_read_u64_be(r).await?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u64_be(w, self.as_u8() as u64).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::astd_write_u64_be(w, self.as_u8() as u64).await?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::NO_RANK => 0x0,
            Self::PARIAH => 0x1,
            Self::OUTLAW => 0x2,
            Self::EXILED => 0x3,
            Self::DISHONORED => 0x4,
            Self::RANK1 => 0x5,
            Self::RANK2 => 0x6,
            Self::RANK3 => 0x7,
            Self::RANK4 => 0x8,
            Self::RANK5 => 0x9,
            Self::RANK6 => 0xa,
            Self::RANK7 => 0xb,
            Self::RANK8 => 0xc,
            Self::RANK9 => 0xd,
            Self::RANK10 => 0xe,
            Self::RANK11 => 0xf,
            Self::RANK12 => 0x10,
            Self::RANK13 => 0x11,
            Self::RANK14 => 0x12,
            Self::FACTION_LEADER => 0x13,
        }
    }

    pub const fn new() -> Self {
        Self::NO_RANK
    }

}

impl ConstantSized for PvpRank {}

impl MaximumPossibleSized for PvpRank {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for PvpRank {
    fn default() -> Self {
        Self::NO_RANK
    }
}

impl std::fmt::Display for PvpRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NO_RANK => f.write_str("NO_RANK"),
            Self::PARIAH => f.write_str("PARIAH"),
            Self::OUTLAW => f.write_str("OUTLAW"),
            Self::EXILED => f.write_str("EXILED"),
            Self::DISHONORED => f.write_str("DISHONORED"),
            Self::RANK1 => f.write_str("RANK1"),
            Self::RANK2 => f.write_str("RANK2"),
            Self::RANK3 => f.write_str("RANK3"),
            Self::RANK4 => f.write_str("RANK4"),
            Self::RANK5 => f.write_str("RANK5"),
            Self::RANK6 => f.write_str("RANK6"),
            Self::RANK7 => f.write_str("RANK7"),
            Self::RANK8 => f.write_str("RANK8"),
            Self::RANK9 => f.write_str("RANK9"),
            Self::RANK10 => f.write_str("RANK10"),
            Self::RANK11 => f.write_str("RANK11"),
            Self::RANK12 => f.write_str("RANK12"),
            Self::RANK13 => f.write_str("RANK13"),
            Self::RANK14 => f.write_str("RANK14"),
            Self::FACTION_LEADER => f.write_str("FACTION_LEADER"),
        }
    }
}

impl TryFrom<u8> for PvpRank {
    type Error = TryFromPvpRankError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NO_RANK),
            1 => Ok(Self::PARIAH),
            2 => Ok(Self::OUTLAW),
            3 => Ok(Self::EXILED),
            4 => Ok(Self::DISHONORED),
            5 => Ok(Self::RANK1),
            6 => Ok(Self::RANK2),
            7 => Ok(Self::RANK3),
            8 => Ok(Self::RANK4),
            9 => Ok(Self::RANK5),
            10 => Ok(Self::RANK6),
            11 => Ok(Self::RANK7),
            12 => Ok(Self::RANK8),
            13 => Ok(Self::RANK9),
            14 => Ok(Self::RANK10),
            15 => Ok(Self::RANK11),
            16 => Ok(Self::RANK12),
            17 => Ok(Self::RANK13),
            18 => Ok(Self::RANK14),
            19 => Ok(Self::FACTION_LEADER),
            _ => Err(TryFromPvpRankError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromPvpRankError {
    value: u8,
}

impl TryFromPvpRankError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum PvpRankError {
    Read(std::io::Error),
    TryFrom(TryFromPvpRankError),
}

impl std::error::Error for PvpRankError {}
impl std::fmt::Display for TryFromPvpRankError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'PvpRank': '{}'", self.value))
    }
}

impl std::fmt::Display for PvpRankError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for PvpRankError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromPvpRankError> for PvpRankError {
    fn from(value: TryFromPvpRankError) -> Self {
        Self::TryFrom(value)
    }
}

