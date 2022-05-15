use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

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
    type Error = ItemQualityError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::POOR),
            1 => Ok(Self::NORMAL),
            2 => Ok(Self::UNCOMMON),
            3 => Ok(Self::RARE),
            4 => Ok(Self::EPIC),
            5 => Ok(Self::LEGENDARY),
            6 => Ok(Self::ARTIFACT),
            _ => Err(ItemQualityError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct ItemQualityError {
    value: u8,
}

impl ItemQualityError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for ItemQualityError {}
impl std::fmt::Display for ItemQualityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'ItemQuality': '{}'", self.value))
    }
}

