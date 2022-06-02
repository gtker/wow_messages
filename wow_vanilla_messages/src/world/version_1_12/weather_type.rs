use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/world/smsg_weather.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/world/smsg_weather.wowm#L3):
/// ```text
/// enum WeatherType : u32 {
///     FINE = 0;
///     RAIN = 1;
///     SNOW = 2;
///     STORM = 3;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum WeatherType {
    FINE,
    RAIN,
    SNOW,
    STORM,
}

impl WeatherType {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::FINE => 0x0,
            Self::RAIN => 0x1,
            Self::SNOW => 0x2,
            Self::STORM => 0x3,
        }
    }

}

impl Default for WeatherType {
    fn default() -> Self {
        Self::FINE
    }
}

impl std::fmt::Display for WeatherType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FINE => f.write_str("FINE"),
            Self::RAIN => f.write_str("RAIN"),
            Self::SNOW => f.write_str("SNOW"),
            Self::STORM => f.write_str("STORM"),
        }
    }
}

impl TryFrom<u32> for WeatherType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::FINE),
            1 => Ok(Self::RAIN),
            2 => Ok(Self::SNOW),
            3 => Ok(Self::STORM),
            v => Err(crate::errors::EnumError::new("WeatherType", v as u32),)
        }
    }
}

