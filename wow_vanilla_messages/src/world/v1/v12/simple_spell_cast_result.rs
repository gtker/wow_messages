use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum SimpleSpellCastResult {
    SUCCESS,
    FAILURE,
}

impl SimpleSpellCastResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::SUCCESS => 0x0,
            Self::FAILURE => 0x2,
        }
    }

}

impl Default for SimpleSpellCastResult {
    fn default() -> Self {
        Self::SUCCESS
    }
}

impl std::fmt::Display for SimpleSpellCastResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SUCCESS => f.write_str("SUCCESS"),
            Self::FAILURE => f.write_str("FAILURE"),
        }
    }
}

impl TryFrom<u8> for SimpleSpellCastResult {
    type Error = SimpleSpellCastResultError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SUCCESS),
            2 => Ok(Self::FAILURE),
            _ => Err(SimpleSpellCastResultError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct SimpleSpellCastResultError {
    value: u8,
}

impl SimpleSpellCastResultError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for SimpleSpellCastResultError {}
impl std::fmt::Display for SimpleSpellCastResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'SimpleSpellCastResult': '{}'", self.value))
    }
}

