use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/quest_common.wowm:77`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/quest_common.wowm#L77):
/// ```text
/// enum QuestFailedReason : u32 {
///     DONT_HAVE_REQ = 0;
///     QUEST_FAILED_LOW_LEVEL = 1;
///     QUEST_FAILED_WRONG_RACE = 6;
///     QUEST_ALREADY_DONE = 7;
///     QUEST_ONLY_ONE_TIMED = 12;
///     QUEST_ALREADY_ON = 13;
///     QUEST_FAILED_EXPANSION = 16;
///     QUEST_ALREADY_ON2 = 18;
///     QUEST_FAILED_MISSING_ITEMS = 21;
///     QUEST_FAILED_NOT_ENOUGH_MONEY = 23;
///     DAILY_QUESTS_REMAINING = 26;
///     QUEST_FAILED_CAIS = 27;
///     DAILY_QUEST_COMPLETED_TODAY = 29;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum QuestFailedReason {
    DontHaveReq,
    /// DESCRIPTION You are not high enough level for that quest.
    ///
    QuestFailedLowLevel,
    /// DESCRIPTION That quest is not available to your race.
    ///
    QuestFailedWrongRace,
    /// DESCRIPTION You have completed that quest.
    ///
    QuestAlreadyDone,
    /// DESCRIPTION You can only be on one timed quest at a time.
    ///
    QuestOnlyOneTimed,
    /// DESCRIPTION You are already on that quest.
    ///
    QuestAlreadyOn,
    /// DESCRIPTION This quest requires an expansion enabled account.
    ///
    QuestFailedExpansion,
    /// DESCRIPTION You are already on that quest.
    ///
    QuestAlreadyOn2,
    /// DESCRIPTION You don't have the required items with you. Check storage.
    ///
    QuestFailedMissingItems,
    /// DESCRIPTION You don't have enough money for that quest.
    ///
    QuestFailedNotEnoughMoney,
    /// DESCRIPTION You have already completed 25 daily quests today.
    ///
    DailyQuestsRemaining,
    /// DESCRIPTION You cannot complete quests once you have reached tired time.
    ///
    QuestFailedCais,
    /// DESCRIPTION You have completed that daily quest today.
    ///
    DailyQuestCompletedToday,
}

impl QuestFailedReason {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::DontHaveReq => 0x0,
            Self::QuestFailedLowLevel => 0x1,
            Self::QuestFailedWrongRace => 0x6,
            Self::QuestAlreadyDone => 0x7,
            Self::QuestOnlyOneTimed => 0xc,
            Self::QuestAlreadyOn => 0xd,
            Self::QuestFailedExpansion => 0x10,
            Self::QuestAlreadyOn2 => 0x12,
            Self::QuestFailedMissingItems => 0x15,
            Self::QuestFailedNotEnoughMoney => 0x17,
            Self::DailyQuestsRemaining => 0x1a,
            Self::QuestFailedCais => 0x1b,
            Self::DailyQuestCompletedToday => 0x1d,
        }
    }

}

impl Default for QuestFailedReason {
    fn default() -> Self {
        Self::DontHaveReq
    }
}

impl std::fmt::Display for QuestFailedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DontHaveReq => f.write_str("DontHaveReq"),
            Self::QuestFailedLowLevel => f.write_str("QuestFailedLowLevel"),
            Self::QuestFailedWrongRace => f.write_str("QuestFailedWrongRace"),
            Self::QuestAlreadyDone => f.write_str("QuestAlreadyDone"),
            Self::QuestOnlyOneTimed => f.write_str("QuestOnlyOneTimed"),
            Self::QuestAlreadyOn => f.write_str("QuestAlreadyOn"),
            Self::QuestFailedExpansion => f.write_str("QuestFailedExpansion"),
            Self::QuestAlreadyOn2 => f.write_str("QuestAlreadyOn2"),
            Self::QuestFailedMissingItems => f.write_str("QuestFailedMissingItems"),
            Self::QuestFailedNotEnoughMoney => f.write_str("QuestFailedNotEnoughMoney"),
            Self::DailyQuestsRemaining => f.write_str("DailyQuestsRemaining"),
            Self::QuestFailedCais => f.write_str("QuestFailedCais"),
            Self::DailyQuestCompletedToday => f.write_str("DailyQuestCompletedToday"),
        }
    }
}

impl TryFrom<u32> for QuestFailedReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DontHaveReq),
            1 => Ok(Self::QuestFailedLowLevel),
            6 => Ok(Self::QuestFailedWrongRace),
            7 => Ok(Self::QuestAlreadyDone),
            12 => Ok(Self::QuestOnlyOneTimed),
            13 => Ok(Self::QuestAlreadyOn),
            16 => Ok(Self::QuestFailedExpansion),
            18 => Ok(Self::QuestAlreadyOn2),
            21 => Ok(Self::QuestFailedMissingItems),
            23 => Ok(Self::QuestFailedNotEnoughMoney),
            26 => Ok(Self::DailyQuestsRemaining),
            27 => Ok(Self::QuestFailedCais),
            29 => Ok(Self::DailyQuestCompletedToday),
            v => Err(crate::errors::EnumError::new("QuestFailedReason", v as u64),)
        }
    }
}

