use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum RaidTargetUpdateType {
    PARTIAL,
    FULL,
}

impl RaidTargetUpdateType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::PARTIAL => 0x0,
            Self::FULL => 0x1,
        }
    }

}

impl ConstantSized for RaidTargetUpdateType {}

impl MaximumPossibleSized for RaidTargetUpdateType {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for RaidTargetUpdateType {
    fn default() -> Self {
        Self::PARTIAL
    }
}

impl std::fmt::Display for RaidTargetUpdateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PARTIAL => f.write_str("PARTIAL"),
            Self::FULL => f.write_str("FULL"),
        }
    }
}

impl TryFrom<u8> for RaidTargetUpdateType {
    type Error = RaidTargetUpdateTypeError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PARTIAL),
            1 => Ok(Self::FULL),
            _ => Err(RaidTargetUpdateTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct RaidTargetUpdateTypeError {
    value: u8,
}

impl RaidTargetUpdateTypeError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for RaidTargetUpdateTypeError {}
impl std::fmt::Display for RaidTargetUpdateTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'RaidTargetUpdateType': '{}'", self.value))
    }
}

