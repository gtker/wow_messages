use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum LootMethod {
    CORPSE,
    PICKPOCKETING,
    FISHING,
    DISENCHANTING,
    SKINNING,
    FISHINGHOLE,
    FISHING_FAIL,
    INSIGNIA,
}

impl LootMethod {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::CORPSE => 0x1,
            Self::PICKPOCKETING => 0x2,
            Self::FISHING => 0x3,
            Self::DISENCHANTING => 0x4,
            Self::SKINNING => 0x6,
            Self::FISHINGHOLE => 0x14,
            Self::FISHING_FAIL => 0x15,
            Self::INSIGNIA => 0x16,
        }
    }

}

impl Default for LootMethod {
    fn default() -> Self {
        Self::CORPSE
    }
}

impl std::fmt::Display for LootMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CORPSE => f.write_str("CORPSE"),
            Self::PICKPOCKETING => f.write_str("PICKPOCKETING"),
            Self::FISHING => f.write_str("FISHING"),
            Self::DISENCHANTING => f.write_str("DISENCHANTING"),
            Self::SKINNING => f.write_str("SKINNING"),
            Self::FISHINGHOLE => f.write_str("FISHINGHOLE"),
            Self::FISHING_FAIL => f.write_str("FISHING_FAIL"),
            Self::INSIGNIA => f.write_str("INSIGNIA"),
        }
    }
}

impl TryFrom<u8> for LootMethod {
    type Error = LootMethodError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::CORPSE),
            2 => Ok(Self::PICKPOCKETING),
            3 => Ok(Self::FISHING),
            4 => Ok(Self::DISENCHANTING),
            6 => Ok(Self::SKINNING),
            20 => Ok(Self::FISHINGHOLE),
            21 => Ok(Self::FISHING_FAIL),
            22 => Ok(Self::INSIGNIA),
            _ => Err(LootMethodError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct LootMethodError {
    value: u8,
}

impl LootMethodError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for LootMethodError {}
impl std::fmt::Display for LootMethodError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'LootMethod': '{}'", self.value))
    }
}

