/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/quest_common.wowm:55`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/quest_common.wowm#L55):
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
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum QuestFailedReason {
    DontHaveReq,
    /// DESCRIPTION You are not high enough level for that quest.
    QuestFailedLowLevel,
    /// DESCRIPTION That quest is not available to your race.
    QuestFailedWrongRace,
    /// DESCRIPTION You have completed that quest.
    QuestAlreadyDone,
    /// DESCRIPTION You can only be on one timed quest at a time.
    QuestOnlyOneTimed,
    /// DESCRIPTION You are already on that quest.
    QuestAlreadyOn,
    /// DESCRIPTION This quest requires an expansion enabled account.
    QuestFailedExpansion,
    /// DESCRIPTION You are already on that quest.
    QuestAlreadyOn2,
    /// DESCRIPTION You don't have the required items with you. Check storage.
    QuestFailedMissingItems,
    /// DESCRIPTION You don't have enough money for that quest.
    QuestFailedNotEnoughMoney,
    /// DESCRIPTION You have already completed 25 daily quests today.
    DailyQuestsRemaining,
    /// DESCRIPTION You cannot complete quests once you have reached tired time.
    QuestFailedCais,
    /// DESCRIPTION You have completed that daily quest today.
    DailyQuestCompletedToday,
}

impl QuestFailedReason {
    pub const fn as_int(&self) -> u32 {
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

    pub const fn variants() -> [Self; 13] {
        [
            Self::DontHaveReq,
            Self::QuestFailedLowLevel,
            Self::QuestFailedWrongRace,
            Self::QuestAlreadyDone,
            Self::QuestOnlyOneTimed,
            Self::QuestAlreadyOn,
            Self::QuestFailedExpansion,
            Self::QuestAlreadyOn2,
            Self::QuestFailedMissingItems,
            Self::QuestFailedNotEnoughMoney,
            Self::DailyQuestsRemaining,
            Self::QuestFailedCais,
            Self::DailyQuestCompletedToday,
        ]
    }

    pub const fn from_int(value: u32) -> Result<Self, crate::errors::EnumError> {
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
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl QuestFailedReason {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::DontHaveReq => "DONT_HAVE_REQ",
            Self::QuestFailedLowLevel => "QUEST_FAILED_LOW_LEVEL",
            Self::QuestFailedWrongRace => "QUEST_FAILED_WRONG_RACE",
            Self::QuestAlreadyDone => "QUEST_ALREADY_DONE",
            Self::QuestOnlyOneTimed => "QUEST_ONLY_ONE_TIMED",
            Self::QuestAlreadyOn => "QUEST_ALREADY_ON",
            Self::QuestFailedExpansion => "QUEST_FAILED_EXPANSION",
            Self::QuestAlreadyOn2 => "QUEST_ALREADY_ON2",
            Self::QuestFailedMissingItems => "QUEST_FAILED_MISSING_ITEMS",
            Self::QuestFailedNotEnoughMoney => "QUEST_FAILED_NOT_ENOUGH_MONEY",
            Self::DailyQuestsRemaining => "DAILY_QUESTS_REMAINING",
            Self::QuestFailedCais => "QUEST_FAILED_CAIS",
            Self::DailyQuestCompletedToday => "DAILY_QUEST_COMPLETED_TODAY",
        }
    }

}

const NAME: &str = "QuestFailedReason";

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
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u8> for QuestFailedReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<u16> for QuestFailedReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<u64> for QuestFailedReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for QuestFailedReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for QuestFailedReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for QuestFailedReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v = u32::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i64> for QuestFailedReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for QuestFailedReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

