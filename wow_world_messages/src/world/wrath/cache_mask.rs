use std::convert::{TryFrom, TryInto};

/// Mask to indicate the variant of the cache
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_account_data_times.wowm:61`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_account_data_times.wowm#L61):
/// ```text
/// enum CacheMask : u32 {
///     GlobalCache = 0x15;
///     PerCharacterCache = 0xEA;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum CacheMask {
    /// Values that are shared between all characters, such as account-wide macros
    ///
    GlobalCache,
    /// Values that are stored per character
    ///
    PerCharacterCache,
}

impl CacheMask {
    pub(crate) const fn as_int(&self) -> u32 {
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
            Self::GlobalCache => f.write_str("Global Cache"),
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
            v => Err(crate::errors::EnumError::new("CacheMask", v as u32),)
        }
    }
}

