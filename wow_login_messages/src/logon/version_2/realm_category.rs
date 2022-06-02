use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_realm/server.wowm:43`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_realm/server.wowm#L43):
/// ```text
/// enum RealmCategory : u8 {
///     DEFAULT = 0x0;
///     ONE = 0x1;
///     TWO = 0x2;
///     THREE = 0x3;
///     FIVE = 0x5;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum RealmCategory {
    DEFAULT,
    ONE,
    TWO,
    THREE,
    FIVE,
}

impl RealmCategory {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::DEFAULT => 0x0,
            Self::ONE => 0x1,
            Self::TWO => 0x2,
            Self::THREE => 0x3,
            Self::FIVE => 0x5,
        }
    }

}

impl Default for RealmCategory {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl std::fmt::Display for RealmCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DEFAULT => f.write_str("DEFAULT"),
            Self::ONE => f.write_str("ONE"),
            Self::TWO => f.write_str("TWO"),
            Self::THREE => f.write_str("THREE"),
            Self::FIVE => f.write_str("FIVE"),
        }
    }
}

impl TryFrom<u8> for RealmCategory {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DEFAULT),
            1 => Ok(Self::ONE),
            2 => Ok(Self::TWO),
            3 => Ok(Self::THREE),
            5 => Ok(Self::FIVE),
            v => Err(crate::errors::EnumError::new("RealmCategory", v as u32),)
        }
    }
}

