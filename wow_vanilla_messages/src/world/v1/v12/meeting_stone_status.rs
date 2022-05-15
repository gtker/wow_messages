use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

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
    type Error = MeetingStoneStatusError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::LEAVE_QUEUE),
            1 => Ok(Self::JOINED_QUEUE),
            2 => Ok(Self::PARTY_MEMBER_LEFT_LFG),
            3 => Ok(Self::PARTY_MEMBER_REMOVED_PARTY_REMOVED),
            4 => Ok(Self::LOOKING_FOR_NEW_PARTY_IN_QUEUE),
            5 => Ok(Self::NONE),
            _ => Err(MeetingStoneStatusError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct MeetingStoneStatusError {
    value: u8,
}

impl MeetingStoneStatusError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for MeetingStoneStatusError {}
impl std::fmt::Display for MeetingStoneStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'MeetingStoneStatus': '{}'", self.value))
    }
}

