use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/quest_common.wowm:154`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/quest_common.wowm#L154):
/// ```text
/// enum QuestGiverStatus : u8 {
///     DIALOG_STATUS_NONE = 0;
///     DIALOG_STATUS_UNAVAILABLE = 1;
///     DIALOG_STATUS_LOW_LEVEL_AVAILABLE = 2;
///     DIALOG_STATUS_LOW_LEVEL_REWARD_REP = 3;
///     DIALOG_STATUS_LOW_LEVEL_AVAILABLE_REP = 4;
///     DIALOG_STATUS_INCOMPLETE = 5;
///     DIALOG_STATUS_REWARD_REP = 6;
///     DIALOG_STATUS_AVAILABLE_REP = 7;
///     DIALOG_STATUS_AVAILABLE = 8;
///     DIALOG_STATUS_REWARD2 = 9;
///     DIALOG_STATUS_REWARD = 10;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum QuestGiverStatus {
    DialogStatusNone,
    DialogStatusUnavailable,
    DialogStatusLowLevelAvailable,
    DialogStatusLowLevelRewardRep,
    DialogStatusLowLevelAvailableRep,
    DialogStatusIncomplete,
    DialogStatusRewardRep,
    DialogStatusAvailableRep,
    DialogStatusAvailable,
    /// no yellow dot on minimap
    ///
    DialogStatusReward2,
    /// yellow dot on minimap
    ///
    DialogStatusReward,
}

impl QuestGiverStatus {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::DialogStatusNone => 0x0,
            Self::DialogStatusUnavailable => 0x1,
            Self::DialogStatusLowLevelAvailable => 0x2,
            Self::DialogStatusLowLevelRewardRep => 0x3,
            Self::DialogStatusLowLevelAvailableRep => 0x4,
            Self::DialogStatusIncomplete => 0x5,
            Self::DialogStatusRewardRep => 0x6,
            Self::DialogStatusAvailableRep => 0x7,
            Self::DialogStatusAvailable => 0x8,
            Self::DialogStatusReward2 => 0x9,
            Self::DialogStatusReward => 0xa,
        }
    }

}

impl Default for QuestGiverStatus {
    fn default() -> Self {
        Self::DialogStatusNone
    }
}

impl std::fmt::Display for QuestGiverStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DialogStatusNone => f.write_str("DialogStatusNone"),
            Self::DialogStatusUnavailable => f.write_str("DialogStatusUnavailable"),
            Self::DialogStatusLowLevelAvailable => f.write_str("DialogStatusLowLevelAvailable"),
            Self::DialogStatusLowLevelRewardRep => f.write_str("DialogStatusLowLevelRewardRep"),
            Self::DialogStatusLowLevelAvailableRep => f.write_str("DialogStatusLowLevelAvailableRep"),
            Self::DialogStatusIncomplete => f.write_str("DialogStatusIncomplete"),
            Self::DialogStatusRewardRep => f.write_str("DialogStatusRewardRep"),
            Self::DialogStatusAvailableRep => f.write_str("DialogStatusAvailableRep"),
            Self::DialogStatusAvailable => f.write_str("DialogStatusAvailable"),
            Self::DialogStatusReward2 => f.write_str("DialogStatusReward2"),
            Self::DialogStatusReward => f.write_str("DialogStatusReward"),
        }
    }
}

impl TryFrom<u8> for QuestGiverStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DialogStatusNone),
            1 => Ok(Self::DialogStatusUnavailable),
            2 => Ok(Self::DialogStatusLowLevelAvailable),
            3 => Ok(Self::DialogStatusLowLevelRewardRep),
            4 => Ok(Self::DialogStatusLowLevelAvailableRep),
            5 => Ok(Self::DialogStatusIncomplete),
            6 => Ok(Self::DialogStatusRewardRep),
            7 => Ok(Self::DialogStatusAvailableRep),
            8 => Ok(Self::DialogStatusAvailable),
            9 => Ok(Self::DialogStatusReward2),
            10 => Ok(Self::DialogStatusReward),
            v => Err(crate::errors::EnumError::new("QuestGiverStatus", v as u32),)
        }
    }
}

