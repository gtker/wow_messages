use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/social_common.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/social_common.wowm#L3):
/// ```text
/// enum GroupLootSetting : u8 {
///     FREE_FOR_ALL = 0;
///     ROUND_ROBIN = 1;
///     MASTER_LOOT = 2;
///     GROUP_LOOT = 3;
///     NEED_BEFORE_GREED = 4;
/// }

/// ```
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
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::FREE_FOR_ALL),
            1 => Ok(Self::ROUND_ROBIN),
            2 => Ok(Self::MASTER_LOOT),
            3 => Ok(Self::GROUP_LOOT),
            4 => Ok(Self::NEED_BEFORE_GREED),
            v => Err(crate::errors::EnumError::new("GroupLootSetting", v as u32),)
        }
    }
}

