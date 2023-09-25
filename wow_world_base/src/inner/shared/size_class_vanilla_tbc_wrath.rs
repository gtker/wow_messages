/// Used in the `CreatureDisplayInfo.dbc`, `CreatureModelData.dbc` and `DeathThudLookups.dbc`.
///
/// This type is not sent over the network, but is used in the game in another way.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external/size_class.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external/size_class.wowm#L2):
/// ```text
/// enum SizeClass : i8 {
///     NONE = -1;
///     SMALL = 0;
///     MEDIUM = 1;
///     LARGE = 2;
///     GIANT = 3;
///     COLOSSAL = 4;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SizeClass {
    None,
    Small,
    Medium,
    Large,
    Giant,
    Colossal,
}

impl SizeClass {
    pub const fn as_int(&self) -> i8 {
        match self {
            Self::None => -1,
            Self::Small => 0x0,
            Self::Medium => 0x1,
            Self::Large => 0x2,
            Self::Giant => 0x3,
            Self::Colossal => 0x4,
        }
    }

    pub const fn variants() -> [Self; 6] {
        [
            Self::None,
            Self::Small,
            Self::Medium,
            Self::Large,
            Self::Giant,
            Self::Colossal,
        ]
    }

    pub const fn from_int(value: i8) -> Result<Self, crate::errors::EnumError> {
        match value {
            -1 => Ok(Self::None),
            0 => Ok(Self::Small),
            1 => Ok(Self::Medium),
            2 => Ok(Self::Large),
            3 => Ok(Self::Giant),
            4 => Ok(Self::Colossal),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl SizeClass {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::Small => "SMALL",
            Self::Medium => "MEDIUM",
            Self::Large => "LARGE",
            Self::Giant => "GIANT",
            Self::Colossal => "COLOSSAL",
        }
    }

}

const NAME: &str = "SizeClass";

impl Default for SizeClass {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for SizeClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::Small => f.write_str("Small"),
            Self::Medium => f.write_str("Medium"),
            Self::Large => f.write_str("Large"),
            Self::Giant => f.write_str("Giant"),
            Self::Colossal => f.write_str("Colossal"),
        }
    }
}

impl TryFrom<i8> for SizeClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u8> for SizeClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        TryInto::<i8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u16> for SizeClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<i8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for SizeClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<i8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for SizeClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<i8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for SizeClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<i8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for SizeClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<i8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for SizeClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<i8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for SizeClass {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<i8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

