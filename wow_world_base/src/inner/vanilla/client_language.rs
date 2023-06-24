/// Used in `GMSurveyCurrentSurvey.dbc`.
///
/// This type is not sent over the network, but is used in the game in another way.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external/client_language.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external/client_language.wowm#L1):
/// ```text
/// enum ClientLanguage : u8 {
///     ENGLISH = 0;
///     KOREAN = 1;
///     FRENCH = 2;
///     GERMAN = 3;
///     CHINESE = 4;
///     TAIWANESE = 5;
///     SPANISH_SPAIN = 6;
///     SPANISH_LATIN_AMERICA = 7;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ClientLanguage {
    English,
    Korean,
    French,
    German,
    Chinese,
    Taiwanese,
    SpanishSpain,
    SpanishLatinAmerica,
}

impl ClientLanguage {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::English => 0x0,
            Self::Korean => 0x1,
            Self::French => 0x2,
            Self::German => 0x3,
            Self::Chinese => 0x4,
            Self::Taiwanese => 0x5,
            Self::SpanishSpain => 0x6,
            Self::SpanishLatinAmerica => 0x7,
        }
    }

    pub const fn variants() -> [Self; 8] {
        [
            Self::English,
            Self::Korean,
            Self::French,
            Self::German,
            Self::Chinese,
            Self::Taiwanese,
            Self::SpanishSpain,
            Self::SpanishLatinAmerica,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl ClientLanguage {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::English => "ENGLISH",
            Self::Korean => "KOREAN",
            Self::French => "FRENCH",
            Self::German => "GERMAN",
            Self::Chinese => "CHINESE",
            Self::Taiwanese => "TAIWANESE",
            Self::SpanishSpain => "SPANISH_SPAIN",
            Self::SpanishLatinAmerica => "SPANISH_LATIN_AMERICA",
        }
    }

}

const NAME: &str = "ClientLanguage";

impl Default for ClientLanguage {
    fn default() -> Self {
        Self::English
    }
}

impl std::fmt::Display for ClientLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::English => f.write_str("English"),
            Self::Korean => f.write_str("Korean"),
            Self::French => f.write_str("French"),
            Self::German => f.write_str("German"),
            Self::Chinese => f.write_str("Chinese"),
            Self::Taiwanese => f.write_str("Taiwanese"),
            Self::SpanishSpain => f.write_str("SpanishSpain"),
            Self::SpanishLatinAmerica => f.write_str("SpanishLatinAmerica"),
        }
    }
}

impl TryFrom<u8> for ClientLanguage {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::English),
            1 => Ok(Self::Korean),
            2 => Ok(Self::French),
            3 => Ok(Self::German),
            4 => Ok(Self::Chinese),
            5 => Ok(Self::Taiwanese),
            6 => Ok(Self::SpanishSpain),
            7 => Ok(Self::SpanishLatinAmerica),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for ClientLanguage {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for ClientLanguage {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for ClientLanguage {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for ClientLanguage {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for ClientLanguage {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for ClientLanguage {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for ClientLanguage {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for ClientLanguage {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

