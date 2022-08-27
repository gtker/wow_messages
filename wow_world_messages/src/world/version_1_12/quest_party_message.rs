use std::convert::{TryFrom, TryInto};

/// cmangos has one instance of this be u32, but both vmangos/mangoszero are u8
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/msg_quest_push_result.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/msg_quest_push_result.wowm#L3):
/// ```text
/// enum QuestPartyMessage : u8 {
///     SHARING_QUEST = 0;
///     CANT_TAKE_QUEST = 1;
///     ACCEPT_QUEST = 2;
///     DECLINE_QUEST = 3;
///     TOO_FAR = 4;
///     BUSY = 5;
///     LOG_FULL = 6;
///     HAVE_QUEST = 7;
///     FINISH_QUEST = 8;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum QuestPartyMessage {
    /// ERR_QUEST_PUSH_SUCCESS_S
    ///
    SharingQuest,
    /// ERR_QUEST_PUSH_INVALID_S
    ///
    CantTakeQuest,
    /// ERR_QUEST_PUSH_ACCEPTED_S
    ///
    AcceptQuest,
    /// ERR_QUEST_PUSH_DECLINED_S
    ///
    DeclineQuest,
    /// removed in 3.x
    ///
    TooFar,
    /// ERR_QUEST_PUSH_BUSY_S
    ///
    Busy,
    /// ERR_QUEST_PUSH_LOG_FULL_S
    ///
    LogFull,
    /// ERR_QUEST_PUSH_ONQUEST_S
    ///
    HaveQuest,
    /// ERR_QUEST_PUSH_ALREADY_DONE_S
    ///
    FinishQuest,
}

impl QuestPartyMessage {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::SharingQuest => 0x0,
            Self::CantTakeQuest => 0x1,
            Self::AcceptQuest => 0x2,
            Self::DeclineQuest => 0x3,
            Self::TooFar => 0x4,
            Self::Busy => 0x5,
            Self::LogFull => 0x6,
            Self::HaveQuest => 0x7,
            Self::FinishQuest => 0x8,
        }
    }

}

impl Default for QuestPartyMessage {
    fn default() -> Self {
        Self::SharingQuest
    }
}

impl std::fmt::Display for QuestPartyMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SharingQuest => f.write_str("SharingQuest"),
            Self::CantTakeQuest => f.write_str("CantTakeQuest"),
            Self::AcceptQuest => f.write_str("AcceptQuest"),
            Self::DeclineQuest => f.write_str("DeclineQuest"),
            Self::TooFar => f.write_str("TooFar"),
            Self::Busy => f.write_str("Busy"),
            Self::LogFull => f.write_str("LogFull"),
            Self::HaveQuest => f.write_str("HaveQuest"),
            Self::FinishQuest => f.write_str("FinishQuest"),
        }
    }
}

impl TryFrom<u8> for QuestPartyMessage {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SharingQuest),
            1 => Ok(Self::CantTakeQuest),
            2 => Ok(Self::AcceptQuest),
            3 => Ok(Self::DeclineQuest),
            4 => Ok(Self::TooFar),
            5 => Ok(Self::Busy),
            6 => Ok(Self::LogFull),
            7 => Ok(Self::HaveQuest),
            8 => Ok(Self::FinishQuest),
            v => Err(crate::errors::EnumError::new("QuestPartyMessage", v as u32),)
        }
    }
}

