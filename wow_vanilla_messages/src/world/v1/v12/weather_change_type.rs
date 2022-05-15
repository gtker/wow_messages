use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum WeatherChangeType {
    SMOOTH,
    INSTANT,
}

impl WeatherChangeType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::SMOOTH => 0x0,
            Self::INSTANT => 0x1,
        }
    }

}

impl Default for WeatherChangeType {
    fn default() -> Self {
        Self::SMOOTH
    }
}

impl std::fmt::Display for WeatherChangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SMOOTH => f.write_str("SMOOTH"),
            Self::INSTANT => f.write_str("INSTANT"),
        }
    }
}

impl TryFrom<u8> for WeatherChangeType {
    type Error = WeatherChangeTypeError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SMOOTH),
            1 => Ok(Self::INSTANT),
            _ => Err(WeatherChangeTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct WeatherChangeTypeError {
    pub value: u8,
}

impl WeatherChangeTypeError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for WeatherChangeTypeError {}
impl std::fmt::Display for WeatherChangeTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'WeatherChangeType': '{}'", self.value))
    }
}

