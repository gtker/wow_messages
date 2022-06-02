use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/social_common.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/social_common.wowm#L11):
/// ```text
/// enum ItemQuality : u8 {
///     POOR = 0;
///     NORMAL = 1;
///     UNCOMMON = 2;
///     RARE = 3;
///     EPIC = 4;
///     LEGENDARY = 5;
///     ARTIFACT = 6;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ItemQuality {
    POOR,
    NORMAL,
    UNCOMMON,
    RARE,
    EPIC,
    LEGENDARY,
    ARTIFACT,
}

impl ItemQuality {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::POOR => 0x0,
            Self::NORMAL => 0x1,
            Self::UNCOMMON => 0x2,
            Self::RARE => 0x3,
            Self::EPIC => 0x4,
            Self::LEGENDARY => 0x5,
            Self::ARTIFACT => 0x6,
        }
    }

}

impl Default for ItemQuality {
    fn default() -> Self {
        Self::POOR
    }
}

impl std::fmt::Display for ItemQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::POOR => f.write_str("POOR"),
            Self::NORMAL => f.write_str("NORMAL"),
            Self::UNCOMMON => f.write_str("UNCOMMON"),
            Self::RARE => f.write_str("RARE"),
            Self::EPIC => f.write_str("EPIC"),
            Self::LEGENDARY => f.write_str("LEGENDARY"),
            Self::ARTIFACT => f.write_str("ARTIFACT"),
        }
    }
}

impl TryFrom<u8> for ItemQuality {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::POOR),
            1 => Ok(Self::NORMAL),
            2 => Ok(Self::UNCOMMON),
            3 => Ok(Self::RARE),
            4 => Ok(Self::EPIC),
            5 => Ok(Self::LEGENDARY),
            6 => Ok(Self::ARTIFACT),
            v => Err(crate::errors::EnumError::new("ItemQuality", v as u32),)
        }
    }
}

