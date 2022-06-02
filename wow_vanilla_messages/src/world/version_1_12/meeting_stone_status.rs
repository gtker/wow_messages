use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_setqueue.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_setqueue.wowm#L3):
/// ```text
/// enum MeetingStoneStatus : u8 {
///     LEAVE_QUEUE = 0;
///     JOINED_QUEUE = 1;
///     PARTY_MEMBER_LEFT_LFG = 2;
///     PARTY_MEMBER_REMOVED_PARTY_REMOVED = 3;
///     LOOKING_FOR_NEW_PARTY_IN_QUEUE = 4;
///     NONE = 5;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum MeetingStoneStatus {
    LEAVE_QUEUE,
    JOINED_QUEUE,
    PARTY_MEMBER_LEFT_LFG,
    PARTY_MEMBER_REMOVED_PARTY_REMOVED,
    LOOKING_FOR_NEW_PARTY_IN_QUEUE,
    NONE,
}

impl MeetingStoneStatus {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::LEAVE_QUEUE => 0x0,
            Self::JOINED_QUEUE => 0x1,
            Self::PARTY_MEMBER_LEFT_LFG => 0x2,
            Self::PARTY_MEMBER_REMOVED_PARTY_REMOVED => 0x3,
            Self::LOOKING_FOR_NEW_PARTY_IN_QUEUE => 0x4,
            Self::NONE => 0x5,
        }
    }

}

impl Default for MeetingStoneStatus {
    fn default() -> Self {
        Self::LEAVE_QUEUE
    }
}

impl std::fmt::Display for MeetingStoneStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LEAVE_QUEUE => f.write_str("LEAVE_QUEUE"),
            Self::JOINED_QUEUE => f.write_str("JOINED_QUEUE"),
            Self::PARTY_MEMBER_LEFT_LFG => f.write_str("PARTY_MEMBER_LEFT_LFG"),
            Self::PARTY_MEMBER_REMOVED_PARTY_REMOVED => f.write_str("PARTY_MEMBER_REMOVED_PARTY_REMOVED"),
            Self::LOOKING_FOR_NEW_PARTY_IN_QUEUE => f.write_str("LOOKING_FOR_NEW_PARTY_IN_QUEUE"),
            Self::NONE => f.write_str("NONE"),
        }
    }
}

impl TryFrom<u8> for MeetingStoneStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::LEAVE_QUEUE),
            1 => Ok(Self::JOINED_QUEUE),
            2 => Ok(Self::PARTY_MEMBER_LEFT_LFG),
            3 => Ok(Self::PARTY_MEMBER_REMOVED_PARTY_REMOVED),
            4 => Ok(Self::LOOKING_FOR_NEW_PARTY_IN_QUEUE),
            5 => Ok(Self::NONE),
            v => Err(crate::errors::EnumError::new("MeetingStoneStatus", v as u32),)
        }
    }
}

