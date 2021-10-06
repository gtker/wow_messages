use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/new4.wowm:119`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/new4.wowm):
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

impl ReadableAndWritable for MeetingStoneStatus {
    type Error = MeetingStoneStatusError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl MeetingStoneStatus {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MeetingStoneStatusError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MeetingStoneStatusError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MeetingStoneStatusError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MeetingStoneStatusError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MeetingStoneStatusError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MeetingStoneStatusError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::LEAVE_QUEUE => 0x0,
            Self::JOINED_QUEUE => 0x1,
            Self::PARTY_MEMBER_LEFT_LFG => 0x2,
            Self::PARTY_MEMBER_REMOVED_PARTY_REMOVED => 0x3,
            Self::LOOKING_FOR_NEW_PARTY_IN_QUEUE => 0x4,
            Self::NONE => 0x5,
        }
    }

    pub const fn new() -> Self {
        Self::LEAVE_QUEUE
    }

}

impl ConstantSized for MeetingStoneStatus {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for MeetingStoneStatus {
    fn maximum_possible_size() -> usize {
        1
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
    type Error = TryFromMeetingStoneStatusError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::LEAVE_QUEUE),
            1 => Ok(Self::JOINED_QUEUE),
            2 => Ok(Self::PARTY_MEMBER_LEFT_LFG),
            3 => Ok(Self::PARTY_MEMBER_REMOVED_PARTY_REMOVED),
            4 => Ok(Self::LOOKING_FOR_NEW_PARTY_IN_QUEUE),
            5 => Ok(Self::NONE),
            _ => Err(TryFromMeetingStoneStatusError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromMeetingStoneStatusError {
    value: u8,
}

impl TryFromMeetingStoneStatusError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum MeetingStoneStatusError {
    Read(std::io::Error),
    TryFrom(TryFromMeetingStoneStatusError),
}

impl std::error::Error for MeetingStoneStatusError {}
impl std::fmt::Display for TryFromMeetingStoneStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'MeetingStoneStatus': '{}'", self.value))
    }
}

impl std::fmt::Display for MeetingStoneStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for MeetingStoneStatusError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromMeetingStoneStatusError> for MeetingStoneStatusError {
    fn from(value: TryFromMeetingStoneStatusError) -> Self {
        Self::TryFrom(value)
    }
}

