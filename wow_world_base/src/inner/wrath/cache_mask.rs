use std::convert::{TryFrom, TryInto};

/// Mask to indicate the variant of the cache
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/cache_mask.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/cache_mask.wowm#L1):
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
    ///
    GlobalCache,
    /// Values that are stored per character
    ///
    PerCharacterCache,
}

impl CacheMask {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::GlobalCache => 0x15,
            Self::PerCharacterCache => 0xea,
        }
    }

}

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
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            21 => Ok(Self::GlobalCache),
            234 => Ok(Self::PerCharacterCache),
            v => Err(crate::errors::EnumError::new("CacheMask", v),)
        }
    }
}

