/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/quest_common.wowm:26`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/quest_common.wowm#L26):
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
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum QuestFailedReason {
    /// this is default case
    DontHaveReq,
    /// You are not high enough level for that quest.
    QuestFailedLowLevel,
    /// That quest is not available to your race.
    QuestFailedWrongRace,
    /// You have completed that quest.
    QuestAlreadyDone,
    /// You can only be on one timed quest at a time.
    QuestOnlyOneTimed,
    /// You are already on that quest.
    QuestAlreadyOn,
    /// This quest requires an expansion enabled account.
    QuestFailedExpansion,
    /// You are already on that quest.
    QuestAlreadyOn2,
    /// You don't have the required items with you. Check storage.
    QuestFailedMissingItems,
    /// You don't have enough money for that quest.
    QuestFailedNotEnoughMoney,
    /// You have already completed 10 daily quests today.
    DailyQuestsRemaining,
    /// You cannot complete quests once you have reached tired time.
    QuestFailedCais,
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
        }
    }

    pub const fn variants() -> [Self; 12] {
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
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u16> for QuestFailedReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
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
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
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

