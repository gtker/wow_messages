use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum TimerType {
    FATIGUE,
    BREATH,
    FEIGNDEATH,
    ENVIRONMENTAL,
}

impl TimerType {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::FATIGUE => 0x0,
            Self::BREATH => 0x1,
            Self::FEIGNDEATH => 0x2,
            Self::ENVIRONMENTAL => 0x3,
        }
    }

}

impl Default for TimerType {
    fn default() -> Self {
        Self::FATIGUE
    }
}

impl std::fmt::Display for TimerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FATIGUE => f.write_str("FATIGUE"),
            Self::BREATH => f.write_str("BREATH"),
            Self::FEIGNDEATH => f.write_str("FEIGNDEATH"),
            Self::ENVIRONMENTAL => f.write_str("ENVIRONMENTAL"),
        }
    }
}

impl TryFrom<u32> for TimerType {
    type Error = TimerTypeError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::FATIGUE),
            1 => Ok(Self::BREATH),
            2 => Ok(Self::FEIGNDEATH),
            3 => Ok(Self::ENVIRONMENTAL),
            _ => Err(TimerTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TimerTypeError {
    value: u32,
}

impl TimerTypeError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::error::Error for TimerTypeError {}
impl std::fmt::Display for TimerTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'TimerType': '{}'", self.value))
    }
}

