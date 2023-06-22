/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/quest_common.wowm:154`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/quest_common.wowm#L154):
/// ```text
/// enum QuestGiverStatus : u8 {
///     NONE = 0;
///     UNAVAILABLE = 1;
///     LOW_LEVEL_AVAILABLE = 2;
///     LOW_LEVEL_REWARD_REP = 3;
///     LOW_LEVEL_AVAILABLE_REP = 4;
///     INCOMPLETE = 5;
///     REWARD_REP = 6;
///     AVAILABLE_REP = 7;
///     AVAILABLE = 8;
///     REWARD2 = 9;
///     REWARD = 10;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum QuestGiverStatus {
    None,
    Unavailable,
    LowLevelAvailable,
    LowLevelRewardRep,
    LowLevelAvailableRep,
    Incomplete,
    RewardRep,
    AvailableRep,
    Available,
    /// no yellow dot on minimap
    Reward2,
    /// yellow dot on minimap
    Reward,
}

impl QuestGiverStatus {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0x0,
            Self::Unavailable => 0x1,
            Self::LowLevelAvailable => 0x2,
            Self::LowLevelRewardRep => 0x3,
            Self::LowLevelAvailableRep => 0x4,
            Self::Incomplete => 0x5,
            Self::RewardRep => 0x6,
            Self::AvailableRep => 0x7,
            Self::Available => 0x8,
            Self::Reward2 => 0x9,
            Self::Reward => 0xa,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl QuestGiverStatus {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::Unavailable => "UNAVAILABLE",
            Self::LowLevelAvailable => "LOW_LEVEL_AVAILABLE",
            Self::LowLevelRewardRep => "LOW_LEVEL_REWARD_REP",
            Self::LowLevelAvailableRep => "LOW_LEVEL_AVAILABLE_REP",
            Self::Incomplete => "INCOMPLETE",
            Self::RewardRep => "REWARD_REP",
            Self::AvailableRep => "AVAILABLE_REP",
            Self::Available => "AVAILABLE",
            Self::Reward2 => "REWARD2",
            Self::Reward => "REWARD",
        }
    }

}

const NAME: &str = "QuestGiverStatus";

impl Default for QuestGiverStatus {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for QuestGiverStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::Unavailable => f.write_str("Unavailable"),
            Self::LowLevelAvailable => f.write_str("LowLevelAvailable"),
            Self::LowLevelRewardRep => f.write_str("LowLevelRewardRep"),
            Self::LowLevelAvailableRep => f.write_str("LowLevelAvailableRep"),
            Self::Incomplete => f.write_str("Incomplete"),
            Self::RewardRep => f.write_str("RewardRep"),
            Self::AvailableRep => f.write_str("AvailableRep"),
            Self::Available => f.write_str("Available"),
            Self::Reward2 => f.write_str("Reward2"),
            Self::Reward => f.write_str("Reward"),
        }
    }
}

impl TryFrom<u8> for QuestGiverStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Unavailable),
            2 => Ok(Self::LowLevelAvailable),
            3 => Ok(Self::LowLevelRewardRep),
            4 => Ok(Self::LowLevelAvailableRep),
            5 => Ok(Self::Incomplete),
            6 => Ok(Self::RewardRep),
            7 => Ok(Self::AvailableRep),
            8 => Ok(Self::Available),
            9 => Ok(Self::Reward2),
            10 => Ok(Self::Reward),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for QuestGiverStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for QuestGiverStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for QuestGiverStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for QuestGiverStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for QuestGiverStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for QuestGiverStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for QuestGiverStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for QuestGiverStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

