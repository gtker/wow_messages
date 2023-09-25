/// Mask to indicate the variant of the cache
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/cache_mask.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/cache_mask.wowm#L2):
/// ```text
/// enum CacheMask : u32 {
///     GLOBAL_CACHE = 0x15;
///     PER_CHARACTER_CACHE = 0xEA;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum CacheMask {
    /// Values that are shared between all characters, such as account-wide macros
    GlobalCache,
    /// Values that are stored per character
    PerCharacterCache,
}

impl CacheMask {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::GlobalCache => 0x15,
            Self::PerCharacterCache => 0xea,
        }
    }

    pub const fn variants() -> [Self; 2] {
        [
            Self::GlobalCache,
            Self::PerCharacterCache,
        ]
    }

    pub const fn from_int(value: u32) -> Result<Self, crate::errors::EnumError> {
        match value {
            21 => Ok(Self::GlobalCache),
            234 => Ok(Self::PerCharacterCache),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl CacheMask {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::GlobalCache => "GLOBAL_CACHE",
            Self::PerCharacterCache => "PER_CHARACTER_CACHE",
        }
    }

}

const NAME: &str = "CacheMask";

impl Default for CacheMask {
    fn default() -> Self {
        Self::GlobalCache
    }
}

impl std::fmt::Display for CacheMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GlobalCache => f.write_str("GlobalCache"),
            Self::PerCharacterCache => f.write_str("Per-Character Cache"),
        }
    }
}

impl TryFrom<u32> for CacheMask {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u8> for CacheMask {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u16> for CacheMask {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for CacheMask {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for CacheMask {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for CacheMask {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for CacheMask {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for CacheMask {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for CacheMask {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

