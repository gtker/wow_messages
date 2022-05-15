use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum BgTypeId {
    NOT_ELIGIBLE,
    QUEUED_FOR_AV,
    QUEUED_FOR_WSG,
    QUEUED_FOR_AB,
    REMOVE_FROM_QUEUE,
}

impl BgTypeId {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::NOT_ELIGIBLE => 0x0,
            Self::QUEUED_FOR_AV => 0x1,
            Self::QUEUED_FOR_WSG => 0x2,
            Self::QUEUED_FOR_AB => 0x3,
            Self::REMOVE_FROM_QUEUE => 0xfffffffe,
        }
    }

}

impl ConstantSized for BgTypeId {}

impl MaximumPossibleSized for BgTypeId {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for BgTypeId {
    fn default() -> Self {
        Self::NOT_ELIGIBLE
    }
}

impl std::fmt::Display for BgTypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NOT_ELIGIBLE => f.write_str("NOT_ELIGIBLE"),
            Self::QUEUED_FOR_AV => f.write_str("QUEUED_FOR_AV"),
            Self::QUEUED_FOR_WSG => f.write_str("QUEUED_FOR_WSG"),
            Self::QUEUED_FOR_AB => f.write_str("QUEUED_FOR_AB"),
            Self::REMOVE_FROM_QUEUE => f.write_str("REMOVE_FROM_QUEUE"),
        }
    }
}

impl TryFrom<u32> for BgTypeId {
    type Error = BgTypeIdError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NOT_ELIGIBLE),
            1 => Ok(Self::QUEUED_FOR_AV),
            2 => Ok(Self::QUEUED_FOR_WSG),
            3 => Ok(Self::QUEUED_FOR_AB),
            4294967294 => Ok(Self::REMOVE_FROM_QUEUE),
            _ => Err(BgTypeIdError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct BgTypeIdError {
    value: u32,
}

impl BgTypeIdError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::error::Error for BgTypeIdError {}
impl std::fmt::Display for BgTypeIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'BgTypeId': '{}'", self.value))
    }
}

