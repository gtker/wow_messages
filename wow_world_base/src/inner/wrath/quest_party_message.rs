/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/msg_quest_push_result.wowm:41`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/msg_quest_push_result.wowm#L41):
/// ```text
/// enum QuestPartyMessage : u8 {
///     MSG_SHARING_QUEST = 0;
///     MSG_CANT_TAKE_QUEST = 1;
///     MSG_ACCEPT_QUEST = 2;
///     MSG_REFUSE_QUEST = 3;
///     MSG_BUSY = 4;
///     MSG_LOG_FULL = 5;
///     MSG_HAVE_QUEST = 6;
///     MSG_FINISH_QUEST = 7;
///     MSG_CANT_BE_SHARED_TODAY = 8;
///     MSG_SHARING_TIMER_EXPIRED = 9;
///     MSG_NOT_IN_PARTY = 10;
///     MSG_DIFFERENT_SERVER_DAILY = 11;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum QuestPartyMessage {
    MsgSharingQuest,
    MsgCantTakeQuest,
    MsgAcceptQuest,
    MsgRefuseQuest,
    MsgBusy,
    MsgLogFull,
    MsgHaveQuest,
    MsgFinishQuest,
    MsgCantBeSharedToday,
    MsgSharingTimerExpired,
    MsgNotInParty,
    MsgDifferentServerDaily,
}

impl QuestPartyMessage {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::MsgSharingQuest => 0x0,
            Self::MsgCantTakeQuest => 0x1,
            Self::MsgAcceptQuest => 0x2,
            Self::MsgRefuseQuest => 0x3,
            Self::MsgBusy => 0x4,
            Self::MsgLogFull => 0x5,
            Self::MsgHaveQuest => 0x6,
            Self::MsgFinishQuest => 0x7,
            Self::MsgCantBeSharedToday => 0x8,
            Self::MsgSharingTimerExpired => 0x9,
            Self::MsgNotInParty => 0xa,
            Self::MsgDifferentServerDaily => 0xb,
        }
    }

}

impl Default for QuestPartyMessage {
    fn default() -> Self {
        Self::MsgSharingQuest
    }
}

impl std::fmt::Display for QuestPartyMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MsgSharingQuest => f.write_str("MsgSharingQuest"),
            Self::MsgCantTakeQuest => f.write_str("MsgCantTakeQuest"),
            Self::MsgAcceptQuest => f.write_str("MsgAcceptQuest"),
            Self::MsgRefuseQuest => f.write_str("MsgRefuseQuest"),
            Self::MsgBusy => f.write_str("MsgBusy"),
            Self::MsgLogFull => f.write_str("MsgLogFull"),
            Self::MsgHaveQuest => f.write_str("MsgHaveQuest"),
            Self::MsgFinishQuest => f.write_str("MsgFinishQuest"),
            Self::MsgCantBeSharedToday => f.write_str("MsgCantBeSharedToday"),
            Self::MsgSharingTimerExpired => f.write_str("MsgSharingTimerExpired"),
            Self::MsgNotInParty => f.write_str("MsgNotInParty"),
            Self::MsgDifferentServerDaily => f.write_str("MsgDifferentServerDaily"),
        }
    }
}

impl TryFrom<u8> for QuestPartyMessage {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::MsgSharingQuest),
            1 => Ok(Self::MsgCantTakeQuest),
            2 => Ok(Self::MsgAcceptQuest),
            3 => Ok(Self::MsgRefuseQuest),
            4 => Ok(Self::MsgBusy),
            5 => Ok(Self::MsgLogFull),
            6 => Ok(Self::MsgHaveQuest),
            7 => Ok(Self::MsgFinishQuest),
            8 => Ok(Self::MsgCantBeSharedToday),
            9 => Ok(Self::MsgSharingTimerExpired),
            10 => Ok(Self::MsgNotInParty),
            11 => Ok(Self::MsgDifferentServerDaily),
            v => Err(crate::errors::EnumError::new("QuestPartyMessage", v as u64),)
        }
    }
}

