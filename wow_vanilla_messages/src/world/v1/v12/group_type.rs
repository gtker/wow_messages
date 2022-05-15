use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GroupType {
    NORMAL,
    RAID,
}

impl GroupType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NORMAL => 0x0,
            Self::RAID => 0x1,
        }
    }

}

impl ConstantSized for GroupType {}

impl MaximumPossibleSized for GroupType {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for GroupType {
    fn default() -> Self {
        Self::NORMAL
    }
}

impl std::fmt::Display for GroupType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NORMAL => f.write_str("NORMAL"),
            Self::RAID => f.write_str("RAID"),
        }
    }
}

impl TryFrom<u8> for GroupType {
    type Error = GroupTypeError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NORMAL),
            1 => Ok(Self::RAID),
            _ => Err(GroupTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct GroupTypeError {
    value: u8,
}

impl GroupTypeError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for GroupTypeError {}
impl std::fmt::Display for GroupTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'GroupType': '{}'", self.value))
    }
}

