use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PetCommandState {
    STAY,
    FOLLOW,
    ATTACK,
    DISMISS,
}

impl PetCommandState {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::STAY => 0x0,
            Self::FOLLOW => 0x1,
            Self::ATTACK => 0x2,
            Self::DISMISS => 0x3,
        }
    }

}

impl Default for PetCommandState {
    fn default() -> Self {
        Self::STAY
    }
}

impl std::fmt::Display for PetCommandState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::STAY => f.write_str("STAY"),
            Self::FOLLOW => f.write_str("FOLLOW"),
            Self::ATTACK => f.write_str("ATTACK"),
            Self::DISMISS => f.write_str("DISMISS"),
        }
    }
}

impl TryFrom<u8> for PetCommandState {
    type Error = PetCommandStateError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::STAY),
            1 => Ok(Self::FOLLOW),
            2 => Ok(Self::ATTACK),
            3 => Ok(Self::DISMISS),
            _ => Err(PetCommandStateError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct PetCommandStateError {
    value: u8,
}

impl PetCommandStateError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for PetCommandStateError {}
impl std::fmt::Display for PetCommandStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'PetCommandState': '{}'", self.value))
    }
}

