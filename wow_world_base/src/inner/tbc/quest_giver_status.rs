/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/quest_common.wowm:100`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/quest_common.wowm#L100):
/// ```text
/// enum QuestGiverStatus : u8 {
///     NONE = 0;
///     UNAVAILABLE = 1;
///     CHAT = 2;
///     INCOMPLETE = 3;
///     REWARD_REP = 4;
///     AVAILABLE_REP = 5;
///     AVAILABLE = 6;
///     REWARD_OLD = 7;
///     REWARD2 = 8;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum QuestGiverStatus {
    None,
    Unavailable,
    Chat,
    Incomplete,
    RewardRep,
    AvailableRep,
    Available,
    /// red dot on minimap
    RewardOld,
    /// yellow dot on minimap
    Reward2,
}

impl QuestGiverStatus {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0x0,
            Self::Unavailable => 0x1,
            Self::Chat => 0x2,
            Self::Incomplete => 0x3,
            Self::RewardRep => 0x4,
            Self::AvailableRep => 0x5,
            Self::Available => 0x6,
            Self::RewardOld => 0x7,
            Self::Reward2 => 0x8,
        }
    }

    pub const fn variants() -> [Self; 9] {
        [
            Self::None,
            Self::Unavailable,
            Self::Chat,
            Self::Incomplete,
            Self::RewardRep,
            Self::AvailableRep,
            Self::Available,
            Self::RewardOld,
            Self::Reward2,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Unavailable),
            2 => Ok(Self::Chat),
            3 => Ok(Self::Incomplete),
            4 => Ok(Self::RewardRep),
            5 => Ok(Self::AvailableRep),
            6 => Ok(Self::Available),
            7 => Ok(Self::RewardOld),
            8 => Ok(Self::Reward2),
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
            Self::Chat => "CHAT",
            Self::Incomplete => "INCOMPLETE",
            Self::RewardRep => "REWARD_REP",
            Self::AvailableRep => "AVAILABLE_REP",
            Self::Available => "AVAILABLE",
            Self::RewardOld => "REWARD_OLD",
            Self::Reward2 => "REWARD2",
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
            Self::Chat => f.write_str("Chat"),
            Self::Incomplete => f.write_str("Incomplete"),
            Self::RewardRep => f.write_str("RewardRep"),
            Self::AvailableRep => f.write_str("AvailableRep"),
            Self::Available => f.write_str("Available"),
            Self::RewardOld => f.write_str("RewardOld"),
            Self::Reward2 => f.write_str("Reward2"),
        }
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

