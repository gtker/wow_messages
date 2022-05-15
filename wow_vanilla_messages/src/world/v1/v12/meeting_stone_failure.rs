use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum MeetingStoneFailure {
    MEETINGSTONE_FAIL_PARTYLEADER,
    MEETINGSTONE_FAIL_FULL_GROUP,
    MEETINGSTONE_FAIL_RAID_GROUP,
}

impl MeetingStoneFailure {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::MEETINGSTONE_FAIL_PARTYLEADER => 0x1,
            Self::MEETINGSTONE_FAIL_FULL_GROUP => 0x2,
            Self::MEETINGSTONE_FAIL_RAID_GROUP => 0x3,
        }
    }

}

impl Default for MeetingStoneFailure {
    fn default() -> Self {
        Self::MEETINGSTONE_FAIL_PARTYLEADER
    }
}

impl std::fmt::Display for MeetingStoneFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MEETINGSTONE_FAIL_PARTYLEADER => f.write_str("MEETINGSTONE_FAIL_PARTYLEADER"),
            Self::MEETINGSTONE_FAIL_FULL_GROUP => f.write_str("MEETINGSTONE_FAIL_FULL_GROUP"),
            Self::MEETINGSTONE_FAIL_RAID_GROUP => f.write_str("MEETINGSTONE_FAIL_RAID_GROUP"),
        }
    }
}

impl TryFrom<u8> for MeetingStoneFailure {
    type Error = MeetingStoneFailureError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::MEETINGSTONE_FAIL_PARTYLEADER),
            2 => Ok(Self::MEETINGSTONE_FAIL_FULL_GROUP),
            3 => Ok(Self::MEETINGSTONE_FAIL_RAID_GROUP),
            _ => Err(MeetingStoneFailureError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct MeetingStoneFailureError {
    value: u8,
}

impl MeetingStoneFailureError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for MeetingStoneFailureError {}
impl std::fmt::Display for MeetingStoneFailureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'MeetingStoneFailure': '{}'", self.value))
    }
}

