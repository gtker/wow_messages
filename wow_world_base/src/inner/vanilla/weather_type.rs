/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/world/smsg_weather.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/world/smsg_weather.wowm#L1):
/// ```text
/// enum WeatherType : u32 {
///     FINE = 0;
///     RAIN = 1;
///     SNOW = 2;
///     STORM = 3;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum WeatherType {
    Fine,
    Rain,
    Snow,
    Storm,
}

impl WeatherType {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Fine => 0x0,
            Self::Rain => 0x1,
            Self::Snow => 0x2,
            Self::Storm => 0x3,
        }
    }

}

impl Default for WeatherType {
    fn default() -> Self {
        Self::Fine
    }
}

impl std::fmt::Display for WeatherType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fine => f.write_str("Fine"),
            Self::Rain => f.write_str("Rain"),
            Self::Snow => f.write_str("Snow"),
            Self::Storm => f.write_str("Storm"),
        }
    }
}

impl TryFrom<u32> for WeatherType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Fine),
            1 => Ok(Self::Rain),
            2 => Ok(Self::Snow),
            3 => Ok(Self::Storm),
            v => Err(crate::errors::EnumError::new("WeatherType", v as u64),)
        }
    }
}

