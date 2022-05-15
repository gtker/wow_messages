use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum TransferAbortReason {
    NONE,
    IS_FULL,
    NOT_FOUND,
    TOO_MANY_INSTANCES,
    ZONE_IS_IN_COMBAT,
}

impl TransferAbortReason {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NONE => 0x0,
            Self::IS_FULL => 0x1,
            Self::NOT_FOUND => 0x2,
            Self::TOO_MANY_INSTANCES => 0x3,
            Self::ZONE_IS_IN_COMBAT => 0x5,
        }
    }

}

impl ConstantSized for TransferAbortReason {}

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
    type Error = TransferAbortReasonError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NONE),
            1 => Ok(Self::IS_FULL),
            2 => Ok(Self::NOT_FOUND),
            3 => Ok(Self::TOO_MANY_INSTANCES),
            5 => Ok(Self::ZONE_IS_IN_COMBAT),
            _ => Err(TransferAbortReasonError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TransferAbortReasonError {
    value: u8,
}

impl TransferAbortReasonError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for TransferAbortReasonError {}
impl std::fmt::Display for TransferAbortReasonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'TransferAbortReason': '{}'", self.value))
    }
}

