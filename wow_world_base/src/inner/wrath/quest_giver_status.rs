/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/quest_common.wowm:116`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/quest_common.wowm#L116):
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
#[derive(Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum QuestGiverStatus {
    #[default]
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

    pub const fn variants() -> [Self; 11] {
        [
            Self::None,
            Self::Unavailable,
            Self::LowLevelAvailable,
            Self::LowLevelRewardRep,
            Self::LowLevelAvailableRep,
            Self::Incomplete,
            Self::RewardRep,
            Self::AvailableRep,
            Self::Available,
            Self::Reward2,
            Self::Reward,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
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
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
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

impl std::fmt::Display for QuestGiverStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::None => "None",
            Self::Unavailable => "Unavailable",
            Self::LowLevelAvailable => "LowLevelAvailable",
            Self::LowLevelRewardRep => "LowLevelRewardRep",
            Self::LowLevelAvailableRep => "LowLevelAvailableRep",
            Self::Incomplete => "Incomplete",
            Self::RewardRep => "RewardRep",
            Self::AvailableRep => "AvailableRep",
            Self::Available => "Available",
            Self::Reward2 => "Reward2",
            Self::Reward => "Reward",
        })
    }
}

impl TryFrom<u8> for QuestGiverStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
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
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
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

