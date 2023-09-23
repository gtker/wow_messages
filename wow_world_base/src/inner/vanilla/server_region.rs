/// Used in `Cfg_Categories.dbc`.
///
/// This type is not sent over the network, but is used in the game in another way.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external/server_region.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external/server_region.wowm#L2):
/// ```text
/// enum ServerRegion : u8 {
///     UNITED_STATES = 1;
///     KOREA = 2;
///     EUROPE = 3;
///     TAIWAN = 4;
///     CHINA = 5;
///     TEST_SERVER = 99;
///     QA_SERVER = 101;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ServerRegion {
    UnitedStates,
    Korea,
    Europe,
    Taiwan,
    China,
    TestServer,
    QaServer,
}

impl ServerRegion {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::UnitedStates => 0x1,
            Self::Korea => 0x2,
            Self::Europe => 0x3,
            Self::Taiwan => 0x4,
            Self::China => 0x5,
            Self::TestServer => 0x63,
            Self::QaServer => 0x65,
        }
    }

    pub const fn variants() -> [Self; 7] {
        [
            Self::UnitedStates,
            Self::Korea,
            Self::Europe,
            Self::Taiwan,
            Self::China,
            Self::TestServer,
            Self::QaServer,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl ServerRegion {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::UnitedStates => "UNITED_STATES",
            Self::Korea => "KOREA",
            Self::Europe => "EUROPE",
            Self::Taiwan => "TAIWAN",
            Self::China => "CHINA",
            Self::TestServer => "TEST_SERVER",
            Self::QaServer => "QA_SERVER",
        }
    }

}

const NAME: &str = "ServerRegion";

impl Default for ServerRegion {
    fn default() -> Self {
        Self::UnitedStates
    }
}

impl std::fmt::Display for ServerRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnitedStates => f.write_str("UnitedStates"),
            Self::Korea => f.write_str("Korea"),
            Self::Europe => f.write_str("Europe"),
            Self::Taiwan => f.write_str("Taiwan"),
            Self::China => f.write_str("China"),
            Self::TestServer => f.write_str("TestServer"),
            Self::QaServer => f.write_str("QaServer"),
        }
    }
}

impl TryFrom<u8> for ServerRegion {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::UnitedStates),
            2 => Ok(Self::Korea),
            3 => Ok(Self::Europe),
            4 => Ok(Self::Taiwan),
            5 => Ok(Self::China),
            99 => Ok(Self::TestServer),
            101 => Ok(Self::QaServer),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for ServerRegion {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for ServerRegion {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for ServerRegion {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for ServerRegion {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for ServerRegion {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for ServerRegion {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for ServerRegion {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for ServerRegion {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

