use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum BattlegroundEndStatus {
    NOT_ENDED,
    ENDED,
}

impl BattlegroundEndStatus {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NOT_ENDED => 0x0,
            Self::ENDED => 0x1,
        }
    }

}

impl Default for BattlegroundEndStatus {
    fn default() -> Self {
        Self::NOT_ENDED
    }
}

impl std::fmt::Display for BattlegroundEndStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NOT_ENDED => f.write_str("NOT_ENDED"),
            Self::ENDED => f.write_str("ENDED"),
        }
    }
}

impl TryFrom<u8> for BattlegroundEndStatus {
    type Error = BattlegroundEndStatusError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NOT_ENDED),
            1 => Ok(Self::ENDED),
            _ => Err(BattlegroundEndStatusError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct BattlegroundEndStatusError {
    pub value: u8,
}

impl BattlegroundEndStatusError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for BattlegroundEndStatusError {}
impl std::fmt::Display for BattlegroundEndStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'BattlegroundEndStatus': '{}'", self.value))
    }
}

