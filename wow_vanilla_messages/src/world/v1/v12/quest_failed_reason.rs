use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum QuestFailedReason {
    DONT_HAVE_REQ,
    QUEST_FAILED_LOW_LEVEL,
    QUEST_FAILED_REQS,
    QUEST_FAILED_INVENTORY_FULL,
    QUEST_FAILED_WRONG_RACE,
    QUEST_ONLY_ONE_TIMED,
    QUEST_ALREADY_ON,
    QUEST_FAILED_DUPLICATE_ITEM,
    QUEST_FAILED_MISSING_ITEMS,
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

impl ConstantSized for QuestFailedReason {}

impl MaximumPossibleSized for QuestFailedReason {
    fn maximum_possible_size() -> usize {
        4
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
    type Error = QuestFailedReasonError;
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
            _ => Err(QuestFailedReasonError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct QuestFailedReasonError {
    value: u32,
}

impl QuestFailedReasonError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::error::Error for QuestFailedReasonError {}
impl std::fmt::Display for QuestFailedReasonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'QuestFailedReason': '{}'", self.value))
    }
}

