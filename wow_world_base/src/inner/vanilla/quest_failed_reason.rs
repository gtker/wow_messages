/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/quest_common.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/quest_common.wowm#L1):
/// ```text
/// enum QuestFailedReason : u32 {
///     DONT_HAVE_REQ = 0;
///     QUEST_FAILED_LOW_LEVEL = 1;
///     QUEST_FAILED_REQS = 2;
///     QUEST_FAILED_INVENTORY_FULL = 4;
///     QUEST_FAILED_WRONG_RACE = 6;
///     QUEST_ONLY_ONE_TIMED = 12;
///     QUEST_ALREADY_ON = 13;
///     QUEST_FAILED_DUPLICATE_ITEM = 17;
///     QUEST_FAILED_MISSING_ITEMS = 20;
///     QUEST_FAILED_NOT_ENOUGH_MONEY = 22;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum QuestFailedReason {
    /// this is default case
    DontHaveReq,
    /// You are not high enough level for that quest.
    QuestFailedLowLevel,
    /// You don't meet the requirements for that quest.
    QuestFailedReqs,
    /// Inventory is full. (Also 50. From SMSG_QUESTGIVER_QUEST_FAILED)
    QuestFailedInventoryFull,
    /// That quest is not available to your race.
    QuestFailedWrongRace,
    /// You can only be on one timed quest at a time.
    QuestOnlyOneTimed,
    /// You are already on that quest.
    QuestAlreadyOn,
    /// Duplicate item found. (From SMSG_QUESTGIVER_QUEST_FAILED)
    QuestFailedDuplicateItem,
    /// You don't have the required items with you. Check storage.
    QuestFailedMissingItems,
    /// You don't have enough money for that quest.
    QuestFailedNotEnoughMoney,
}

impl QuestFailedReason {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::DontHaveReq => 0x0,
            Self::QuestFailedLowLevel => 0x1,
            Self::QuestFailedReqs => 0x2,
            Self::QuestFailedInventoryFull => 0x4,
            Self::QuestFailedWrongRace => 0x6,
            Self::QuestOnlyOneTimed => 0xc,
            Self::QuestAlreadyOn => 0xd,
            Self::QuestFailedDuplicateItem => 0x11,
            Self::QuestFailedMissingItems => 0x14,
            Self::QuestFailedNotEnoughMoney => 0x16,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl QuestFailedReason {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::DontHaveReq => "DONT_HAVE_REQ",
            Self::QuestFailedLowLevel => "QUEST_FAILED_LOW_LEVEL",
            Self::QuestFailedReqs => "QUEST_FAILED_REQS",
            Self::QuestFailedInventoryFull => "QUEST_FAILED_INVENTORY_FULL",
            Self::QuestFailedWrongRace => "QUEST_FAILED_WRONG_RACE",
            Self::QuestOnlyOneTimed => "QUEST_ONLY_ONE_TIMED",
            Self::QuestAlreadyOn => "QUEST_ALREADY_ON",
            Self::QuestFailedDuplicateItem => "QUEST_FAILED_DUPLICATE_ITEM",
            Self::QuestFailedMissingItems => "QUEST_FAILED_MISSING_ITEMS",
            Self::QuestFailedNotEnoughMoney => "QUEST_FAILED_NOT_ENOUGH_MONEY",
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
            Self::QuestFailedReqs => f.write_str("QuestFailedReqs"),
            Self::QuestFailedInventoryFull => f.write_str("QuestFailedInventoryFull"),
            Self::QuestFailedWrongRace => f.write_str("QuestFailedWrongRace"),
            Self::QuestOnlyOneTimed => f.write_str("QuestOnlyOneTimed"),
            Self::QuestAlreadyOn => f.write_str("QuestAlreadyOn"),
            Self::QuestFailedDuplicateItem => f.write_str("QuestFailedDuplicateItem"),
            Self::QuestFailedMissingItems => f.write_str("QuestFailedMissingItems"),
            Self::QuestFailedNotEnoughMoney => f.write_str("QuestFailedNotEnoughMoney"),
        }
    }
}

impl TryFrom<u32> for QuestFailedReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DontHaveReq),
            1 => Ok(Self::QuestFailedLowLevel),
            2 => Ok(Self::QuestFailedReqs),
            4 => Ok(Self::QuestFailedInventoryFull),
            6 => Ok(Self::QuestFailedWrongRace),
            12 => Ok(Self::QuestOnlyOneTimed),
            13 => Ok(Self::QuestAlreadyOn),
            17 => Ok(Self::QuestFailedDuplicateItem),
            20 => Ok(Self::QuestFailedMissingItems),
            22 => Ok(Self::QuestFailedNotEnoughMoney),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
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

