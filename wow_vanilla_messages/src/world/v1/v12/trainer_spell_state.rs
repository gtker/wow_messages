use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum TrainerSpellState {
    GREEN,
    RED,
    GRAY,
}

impl TrainerSpellState {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::GREEN => 0x0,
            Self::RED => 0x1,
            Self::GRAY => 0x2,
        }
    }

}

impl Default for TrainerSpellState {
    fn default() -> Self {
        Self::GREEN
    }
}

impl std::fmt::Display for TrainerSpellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GREEN => f.write_str("GREEN"),
            Self::RED => f.write_str("RED"),
            Self::GRAY => f.write_str("GRAY"),
        }
    }
}

impl TryFrom<u8> for TrainerSpellState {
    type Error = TrainerSpellStateError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::GREEN),
            1 => Ok(Self::RED),
            2 => Ok(Self::GRAY),
            _ => Err(TrainerSpellStateError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TrainerSpellStateError {
    value: u8,
}

impl TrainerSpellStateError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for TrainerSpellStateError {}
impl std::fmt::Display for TrainerSpellStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'TrainerSpellState': '{}'", self.value))
    }
}

