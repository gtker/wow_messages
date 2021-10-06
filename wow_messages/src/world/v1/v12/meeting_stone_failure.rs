use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/new4.wowm:277`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/new4.wowm):
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

impl ReadableAndWritable for MeetingStoneFailure {
    type Error = MeetingStoneFailureError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl MeetingStoneFailure {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MeetingStoneFailureError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MeetingStoneFailureError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MeetingStoneFailureError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MeetingStoneFailureError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MeetingStoneFailureError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MeetingStoneFailureError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::MEETINGSTONE_FAIL_PARTYLEADER => 0x1,
            Self::MEETINGSTONE_FAIL_FULL_GROUP => 0x2,
            Self::MEETINGSTONE_FAIL_RAID_GROUP => 0x3,
        }
    }

    pub const fn new() -> Self {
        Self::MEETINGSTONE_FAIL_PARTYLEADER
    }

}

impl ConstantSized for MeetingStoneFailure {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for MeetingStoneFailure {
    fn maximum_possible_size() -> usize {
        1
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
    type Error = TryFromMeetingStoneFailureError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::MEETINGSTONE_FAIL_PARTYLEADER),
            2 => Ok(Self::MEETINGSTONE_FAIL_FULL_GROUP),
            3 => Ok(Self::MEETINGSTONE_FAIL_RAID_GROUP),
            _ => Err(TryFromMeetingStoneFailureError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromMeetingStoneFailureError {
    value: u8,
}

impl TryFromMeetingStoneFailureError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum MeetingStoneFailureError {
    Read(std::io::Error),
    TryFrom(TryFromMeetingStoneFailureError),
}

impl std::error::Error for MeetingStoneFailureError {}
impl std::fmt::Display for TryFromMeetingStoneFailureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'MeetingStoneFailure': '{}'", self.value))
    }
}

impl std::fmt::Display for MeetingStoneFailureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for MeetingStoneFailureError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromMeetingStoneFailureError> for MeetingStoneFailureError {
    fn from(value: TryFromMeetingStoneFailureError) -> Self {
        Self::TryFrom(value)
    }
}

