use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum DuelWinnerReason {
    WON,
    FLED,
}

impl DuelWinnerReason {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::WON => 0x0,
            Self::FLED => 0x1,
        }
    }

}

impl Default for DuelWinnerReason {
    fn default() -> Self {
        Self::WON
    }
}

impl std::fmt::Display for DuelWinnerReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WON => f.write_str("WON"),
            Self::FLED => f.write_str("FLED"),
        }
    }
}

impl TryFrom<u8> for DuelWinnerReason {
    type Error = DuelWinnerReasonError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::WON),
            1 => Ok(Self::FLED),
            _ => Err(DuelWinnerReasonError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct DuelWinnerReasonError {
    pub value: u8,
}

impl DuelWinnerReasonError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for DuelWinnerReasonError {}
impl std::fmt::Display for DuelWinnerReasonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'DuelWinnerReason': '{}'", self.value))
    }
}

