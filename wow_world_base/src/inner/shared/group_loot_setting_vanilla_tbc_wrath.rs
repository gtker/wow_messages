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
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum GroupLootSetting {
    FreeForAll,
    RoundRobin,
    MasterLoot,
    GroupLoot,
    NeedBeforeGreed,
}

impl GroupLootSetting {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::FreeForAll => 0x0,
            Self::RoundRobin => 0x1,
            Self::MasterLoot => 0x2,
            Self::GroupLoot => 0x3,
            Self::NeedBeforeGreed => 0x4,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl GroupLootSetting {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::FreeForAll => "FREE_FOR_ALL",
            Self::RoundRobin => "ROUND_ROBIN",
            Self::MasterLoot => "MASTER_LOOT",
            Self::GroupLoot => "GROUP_LOOT",
            Self::NeedBeforeGreed => "NEED_BEFORE_GREED",
        }
    }

}

impl Default for GroupLootSetting {
    fn default() -> Self {
        Self::FreeForAll
    }
}

impl std::fmt::Display for GroupLootSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FreeForAll => f.write_str("FreeForAll"),
            Self::RoundRobin => f.write_str("RoundRobin"),
            Self::MasterLoot => f.write_str("MasterLoot"),
            Self::GroupLoot => f.write_str("GroupLoot"),
            Self::NeedBeforeGreed => f.write_str("NeedBeforeGreed"),
        }
    }
}

impl TryFrom<u8> for GroupLootSetting {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::FreeForAll),
            1 => Ok(Self::RoundRobin),
            2 => Ok(Self::MasterLoot),
            3 => Ok(Self::GroupLoot),
            4 => Ok(Self::NeedBeforeGreed),
            v => Err(crate::errors::EnumError::new("GroupLootSetting", v.into()),)
        }
    }
}

