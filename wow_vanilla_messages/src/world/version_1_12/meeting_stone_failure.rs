use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_joinfailed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_joinfailed.wowm#L3):
/// ```text
/// enum MeetingStoneFailure : u8 {
///     MEETINGSTONE_FAIL_PARTYLEADER = 1;
///     MEETINGSTONE_FAIL_FULL_GROUP = 2;
///     MEETINGSTONE_FAIL_RAID_GROUP = 3;
/// }

/// ```
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
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::MEETINGSTONE_FAIL_PARTYLEADER),
            2 => Ok(Self::MEETINGSTONE_FAIL_FULL_GROUP),
            3 => Ok(Self::MEETINGSTONE_FAIL_RAID_GROUP),
            v => Err(crate::errors::EnumError::new("MeetingStoneFailure", v as u32),)
        }
    }
}

