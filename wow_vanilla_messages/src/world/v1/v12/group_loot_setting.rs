use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GroupLootSetting {
    FREE_FOR_ALL,
    ROUND_ROBIN,
    MASTER_LOOT,
    GROUP_LOOT,
    NEED_BEFORE_GREED,
}

impl GroupLootSetting {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::FREE_FOR_ALL => 0x0,
            Self::ROUND_ROBIN => 0x1,
            Self::MASTER_LOOT => 0x2,
            Self::GROUP_LOOT => 0x3,
            Self::NEED_BEFORE_GREED => 0x4,
        }
    }

}

impl ConstantSized for GroupLootSetting {}

impl MaximumPossibleSized for GroupLootSetting {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for GroupLootSetting {
    fn default() -> Self {
        Self::FREE_FOR_ALL
    }
}

impl std::fmt::Display for GroupLootSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FREE_FOR_ALL => f.write_str("FREE_FOR_ALL"),
            Self::ROUND_ROBIN => f.write_str("ROUND_ROBIN"),
            Self::MASTER_LOOT => f.write_str("MASTER_LOOT"),
            Self::GROUP_LOOT => f.write_str("GROUP_LOOT"),
            Self::NEED_BEFORE_GREED => f.write_str("NEED_BEFORE_GREED"),
        }
    }
}

impl TryFrom<u8> for GroupLootSetting {
    type Error = GroupLootSettingError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::FREE_FOR_ALL),
            1 => Ok(Self::ROUND_ROBIN),
            2 => Ok(Self::MASTER_LOOT),
            3 => Ok(Self::GROUP_LOOT),
            4 => Ok(Self::NEED_BEFORE_GREED),
            _ => Err(GroupLootSettingError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct GroupLootSettingError {
    value: u8,
}

impl GroupLootSettingError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for GroupLootSettingError {}
impl std::fmt::Display for GroupLootSettingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'GroupLootSetting': '{}'", self.value))
    }
}

