use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/quest_common.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/quest_common.wowm#L3):
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
pub enum QuestFailedReason {
    /// this is default case
    ///
    DONT_HAVE_REQ,
    /// You are not high enough level for that quest.
    ///
    QUEST_FAILED_LOW_LEVEL,
    /// You don't meet the requirements for that quest.
    ///
    QUEST_FAILED_REQS,
    /// Inventory is full. (Also 50. From SMSG_QUESTGIVER_QUEST_FAILED)
    ///
    QUEST_FAILED_INVENTORY_FULL,
    /// That quest is not available to your race.
    ///
    QUEST_FAILED_WRONG_RACE,
    /// You can only be on one timed quest at a time.
    ///
    QUEST_ONLY_ONE_TIMED,
    /// You are already on that quest.
    ///
    QUEST_ALREADY_ON,
    /// Duplicate item found. (From SMSG_QUESTGIVER_QUEST_FAILED)
    ///
    QUEST_FAILED_DUPLICATE_ITEM,
    /// You don't have the required items with you. Check storage.
    ///
    QUEST_FAILED_MISSING_ITEMS,
    /// You don't have enough money for that quest.
    ///
    QUEST_FAILED_NOT_ENOUGH_MONEY,
}

impl QuestFailedReason {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::DONT_HAVE_REQ => 0x0,
            Self::QUEST_FAILED_LOW_LEVEL => 0x1,
            Self::QUEST_FAILED_REQS => 0x2,
            Self::QUEST_FAILED_INVENTORY_FULL => 0x4,
            Self::QUEST_FAILED_WRONG_RACE => 0x6,
            Self::QUEST_ONLY_ONE_TIMED => 0xc,
            Self::QUEST_ALREADY_ON => 0xd,
            Self::QUEST_FAILED_DUPLICATE_ITEM => 0x11,
            Self::QUEST_FAILED_MISSING_ITEMS => 0x14,
            Self::QUEST_FAILED_NOT_ENOUGH_MONEY => 0x16,
        }
    }

}

impl Default for QuestFailedReason {
    fn default() -> Self {
        Self::DONT_HAVE_REQ
    }
}

impl std::fmt::Display for QuestFailedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DONT_HAVE_REQ => f.write_str("DONT_HAVE_REQ"),
            Self::QUEST_FAILED_LOW_LEVEL => f.write_str("QUEST_FAILED_LOW_LEVEL"),
            Self::QUEST_FAILED_REQS => f.write_str("QUEST_FAILED_REQS"),
            Self::QUEST_FAILED_INVENTORY_FULL => f.write_str("QUEST_FAILED_INVENTORY_FULL"),
            Self::QUEST_FAILED_WRONG_RACE => f.write_str("QUEST_FAILED_WRONG_RACE"),
            Self::QUEST_ONLY_ONE_TIMED => f.write_str("QUEST_ONLY_ONE_TIMED"),
            Self::QUEST_ALREADY_ON => f.write_str("QUEST_ALREADY_ON"),
            Self::QUEST_FAILED_DUPLICATE_ITEM => f.write_str("QUEST_FAILED_DUPLICATE_ITEM"),
            Self::QUEST_FAILED_MISSING_ITEMS => f.write_str("QUEST_FAILED_MISSING_ITEMS"),
            Self::QUEST_FAILED_NOT_ENOUGH_MONEY => f.write_str("QUEST_FAILED_NOT_ENOUGH_MONEY"),
        }
    }
}

impl TryFrom<u32> for QuestFailedReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DONT_HAVE_REQ),
            1 => Ok(Self::QUEST_FAILED_LOW_LEVEL),
            2 => Ok(Self::QUEST_FAILED_REQS),
            4 => Ok(Self::QUEST_FAILED_INVENTORY_FULL),
            6 => Ok(Self::QUEST_FAILED_WRONG_RACE),
            12 => Ok(Self::QUEST_ONLY_ONE_TIMED),
            13 => Ok(Self::QUEST_ALREADY_ON),
            17 => Ok(Self::QUEST_FAILED_DUPLICATE_ITEM),
            20 => Ok(Self::QUEST_FAILED_MISSING_ITEMS),
            22 => Ok(Self::QUEST_FAILED_NOT_ENOUGH_MONEY),
            v => Err(crate::errors::EnumError::new("QuestFailedReason", v as u32),)
        }
    }
}

