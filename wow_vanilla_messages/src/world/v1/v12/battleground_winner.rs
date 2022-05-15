use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum BattlegroundWinner {
    HORDE,
    ALLIANCE,
    NONE,
}

impl BattlegroundWinner {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::HORDE => 0x0,
            Self::ALLIANCE => 0x1,
            Self::NONE => 0x2,
        }
    }

}

impl Default for BattlegroundWinner {
    fn default() -> Self {
        Self::HORDE
    }
}

impl std::fmt::Display for BattlegroundWinner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HORDE => f.write_str("HORDE"),
            Self::ALLIANCE => f.write_str("ALLIANCE"),
            Self::NONE => f.write_str("NONE"),
        }
    }
}

impl TryFrom<u8> for BattlegroundWinner {
    type Error = BattlegroundWinnerError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::HORDE),
            1 => Ok(Self::ALLIANCE),
            2 => Ok(Self::NONE),
            _ => Err(BattlegroundWinnerError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct BattlegroundWinnerError {
    value: u8,
}

impl BattlegroundWinnerError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for BattlegroundWinnerError {}
impl std::fmt::Display for BattlegroundWinnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'BattlegroundWinner': '{}'", self.value))
    }
}

