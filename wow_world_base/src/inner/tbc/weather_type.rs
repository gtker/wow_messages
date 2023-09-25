/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/world/smsg_weather.wowm:27`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/world/smsg_weather.wowm#L27):
/// ```text
/// enum WeatherType : u32 {
///     FINE = 0;
///     LIGHT_RAIN = 3;
///     MEDIUM_RAIN = 4;
///     HEAVY_RAIN = 5;
///     LIGHT_SNOW = 6;
///     MEDIUM_SNOW = 7;
///     HEAVY_SNOW = 8;
///     LIGHT_SANDSTORM = 22;
///     MEDIUM_SANDSTORM = 41;
///     HEAVY_SANDSTORM = 42;
///     THUNDERS = 86;
///     BLACKRAIN = 90;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum WeatherType {
    Fine,
    LightRain,
    MediumRain,
    HeavyRain,
    LightSnow,
    MediumSnow,
    HeavySnow,
    LightSandstorm,
    MediumSandstorm,
    HeavySandstorm,
    Thunders,
    Blackrain,
}

impl WeatherType {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Fine => 0x0,
            Self::LightRain => 0x3,
            Self::MediumRain => 0x4,
            Self::HeavyRain => 0x5,
            Self::LightSnow => 0x6,
            Self::MediumSnow => 0x7,
            Self::HeavySnow => 0x8,
            Self::LightSandstorm => 0x16,
            Self::MediumSandstorm => 0x29,
            Self::HeavySandstorm => 0x2a,
            Self::Thunders => 0x56,
            Self::Blackrain => 0x5a,
        }
    }

    pub const fn variants() -> [Self; 12] {
        [
            Self::Fine,
            Self::LightRain,
            Self::MediumRain,
            Self::HeavyRain,
            Self::LightSnow,
            Self::MediumSnow,
            Self::HeavySnow,
            Self::LightSandstorm,
            Self::MediumSandstorm,
            Self::HeavySandstorm,
            Self::Thunders,
            Self::Blackrain,
        ]
    }

    pub const fn from_int(value: u32) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Fine),
            3 => Ok(Self::LightRain),
            4 => Ok(Self::MediumRain),
            5 => Ok(Self::HeavyRain),
            6 => Ok(Self::LightSnow),
            7 => Ok(Self::MediumSnow),
            8 => Ok(Self::HeavySnow),
            22 => Ok(Self::LightSandstorm),
            41 => Ok(Self::MediumSandstorm),
            42 => Ok(Self::HeavySandstorm),
            86 => Ok(Self::Thunders),
            90 => Ok(Self::Blackrain),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl WeatherType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Fine => "FINE",
            Self::LightRain => "LIGHT_RAIN",
            Self::MediumRain => "MEDIUM_RAIN",
            Self::HeavyRain => "HEAVY_RAIN",
            Self::LightSnow => "LIGHT_SNOW",
            Self::MediumSnow => "MEDIUM_SNOW",
            Self::HeavySnow => "HEAVY_SNOW",
            Self::LightSandstorm => "LIGHT_SANDSTORM",
            Self::MediumSandstorm => "MEDIUM_SANDSTORM",
            Self::HeavySandstorm => "HEAVY_SANDSTORM",
            Self::Thunders => "THUNDERS",
            Self::Blackrain => "BLACKRAIN",
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
            Self::LightRain => f.write_str("LightRain"),
            Self::MediumRain => f.write_str("MediumRain"),
            Self::HeavyRain => f.write_str("HeavyRain"),
            Self::LightSnow => f.write_str("LightSnow"),
            Self::MediumSnow => f.write_str("MediumSnow"),
            Self::HeavySnow => f.write_str("HeavySnow"),
            Self::LightSandstorm => f.write_str("LightSandstorm"),
            Self::MediumSandstorm => f.write_str("MediumSandstorm"),
            Self::HeavySandstorm => f.write_str("HeavySandstorm"),
            Self::Thunders => f.write_str("Thunders"),
            Self::Blackrain => f.write_str("Blackrain"),
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
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u16> for WeatherType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
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
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
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

