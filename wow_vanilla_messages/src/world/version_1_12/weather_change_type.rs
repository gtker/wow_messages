use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/world/smsg_weather.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/world/smsg_weather.wowm#L10):
/// ```text
/// enum WeatherChangeType : u8 {
///     SMOOTH = 0;
///     INSTANT = 1;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum WeatherChangeType {
    Smooth,
    Instant,
}

impl WeatherChangeType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Smooth => 0x0,
            Self::Instant => 0x1,
        }
    }

}

impl Default for WeatherChangeType {
    fn default() -> Self {
        Self::Smooth
    }
}

impl std::fmt::Display for WeatherChangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Smooth => f.write_str("Smooth"),
            Self::Instant => f.write_str("Instant"),
        }
    }
}

impl TryFrom<u8> for WeatherChangeType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Smooth),
            1 => Ok(Self::Instant),
            v => Err(crate::errors::EnumError::new("WeatherChangeType", v as u32),)
        }
    }
}

