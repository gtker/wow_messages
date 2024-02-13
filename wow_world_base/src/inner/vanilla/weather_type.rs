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

    pub const fn variants() -> [Self; 4] {
        [
            Self::Fine,
            Self::Rain,
            Self::Snow,
            Self::Storm,
        ]
    }

    pub const fn from_int(value: u32) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Fine),
            1 => Ok(Self::Rain),
            2 => Ok(Self::Snow),
            3 => Ok(Self::Storm),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl WeatherType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Fine => "FINE",
            Self::Rain => "RAIN",
            Self::Snow => "SNOW",
            Self::Storm => "STORM",
        }
    }

}

const NAME: &str = "WeatherType";

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
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u8> for WeatherType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<u16> for WeatherType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<u64> for WeatherType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for WeatherType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for WeatherType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for WeatherType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v = u32::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i64> for WeatherType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for WeatherType {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

