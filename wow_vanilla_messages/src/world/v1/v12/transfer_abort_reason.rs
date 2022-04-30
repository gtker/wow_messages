use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use crate::AsyncReadWrite;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum TransferAbortReason {
    NONE,
    IS_FULL,
    NOT_FOUND,
    TOO_MANY_INSTANCES,
    ZONE_IS_IN_COMBAT,
}

impl ReadableAndWritable for TransferAbortReason {
    type Error = TransferAbortReasonError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

#[cfg(any(feature = "async_tokio", feature = "async_std"))]
#[async_trait]
impl AsyncReadWrite for TransferAbortReason {
    type Error = TransferAbortReasonError;

    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::tokio_read_u8_le(r).await?;

        Ok(a.try_into()?)
    }

}

impl TransferAbortReason {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, TransferAbortReasonError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, TransferAbortReasonError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, TransferAbortReasonError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, TransferAbortReasonError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, TransferAbortReasonError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, TransferAbortReasonError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::NONE => 0x0,
            Self::IS_FULL => 0x1,
            Self::NOT_FOUND => 0x2,
            Self::TOO_MANY_INSTANCES => 0x3,
            Self::ZONE_IS_IN_COMBAT => 0x5,
        }
    }

    pub const fn new() -> Self {
        Self::NONE
    }

}

impl ConstantSized for TransferAbortReason {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for TransferAbortReason {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for TransferAbortReason {
    fn default() -> Self {
        Self::NONE
    }
}

impl std::fmt::Display for TransferAbortReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NONE => f.write_str("NONE"),
            Self::IS_FULL => f.write_str("IS_FULL"),
            Self::NOT_FOUND => f.write_str("NOT_FOUND"),
            Self::TOO_MANY_INSTANCES => f.write_str("TOO_MANY_INSTANCES"),
            Self::ZONE_IS_IN_COMBAT => f.write_str("ZONE_IS_IN_COMBAT"),
        }
    }
}

impl TryFrom<u8> for TransferAbortReason {
    type Error = TryFromTransferAbortReasonError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NONE),
            1 => Ok(Self::IS_FULL),
            2 => Ok(Self::NOT_FOUND),
            3 => Ok(Self::TOO_MANY_INSTANCES),
            5 => Ok(Self::ZONE_IS_IN_COMBAT),
            _ => Err(TryFromTransferAbortReasonError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromTransferAbortReasonError {
    value: u8,
}

impl TryFromTransferAbortReasonError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum TransferAbortReasonError {
    Read(std::io::Error),
    TryFrom(TryFromTransferAbortReasonError),
}

impl std::error::Error for TransferAbortReasonError {}
impl std::fmt::Display for TryFromTransferAbortReasonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'TransferAbortReason': '{}'", self.value))
    }
}

impl std::fmt::Display for TransferAbortReasonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for TransferAbortReasonError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromTransferAbortReasonError> for TransferAbortReasonError {
    fn from(value: TryFromTransferAbortReasonError) -> Self {
        Self::TryFrom(value)
    }
}

