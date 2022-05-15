use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PetTalkReason {
    SPECIAL_SPELL,
    ATTACK,
}

impl PetTalkReason {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::SPECIAL_SPELL => 0x0,
            Self::ATTACK => 0x1,
        }
    }

}

impl Default for PetTalkReason {
    fn default() -> Self {
        Self::SPECIAL_SPELL
    }
}

impl std::fmt::Display for PetTalkReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SPECIAL_SPELL => f.write_str("SPECIAL_SPELL"),
            Self::ATTACK => f.write_str("ATTACK"),
        }
    }
}

impl TryFrom<u32> for PetTalkReason {
    type Error = PetTalkReasonError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SPECIAL_SPELL),
            1 => Ok(Self::ATTACK),
            _ => Err(PetTalkReasonError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct PetTalkReasonError {
    value: u32,
}

impl PetTalkReasonError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::error::Error for PetTalkReasonError {}
impl std::fmt::Display for PetTalkReasonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'PetTalkReason': '{}'", self.value))
    }
}

