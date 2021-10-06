use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/add_messages.wowm:362`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/add_messages.wowm):
/// ```text
/// enum PartyResult : u8 {
///     SUCCESS = 0;
///     BAD_PLAYER_NAME = 1;
///     TARGET_NOT_IN_GROUP = 2;
///     GROUP_FULL = 3;
///     ALREADY_IN_GROUP = 4;
///     NOT_IN_GROUP = 5;
///     NOT_LEADER = 6;
///     PLAYER_WRONG_FACTION = 7;
///     IGNORING_YOU = 8;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PartyResult {
    SUCCESS,
    BAD_PLAYER_NAME,
    TARGET_NOT_IN_GROUP,
    GROUP_FULL,
    ALREADY_IN_GROUP,
    NOT_IN_GROUP,
    NOT_LEADER,
    PLAYER_WRONG_FACTION,
    IGNORING_YOU,
}

impl ReadableAndWritable for PartyResult {
    type Error = PartyResultError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl PartyResult {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PartyResultError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PartyResultError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PartyResultError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PartyResultError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PartyResultError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PartyResultError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::SUCCESS => 0x0,
            Self::BAD_PLAYER_NAME => 0x1,
            Self::TARGET_NOT_IN_GROUP => 0x2,
            Self::GROUP_FULL => 0x3,
            Self::ALREADY_IN_GROUP => 0x4,
            Self::NOT_IN_GROUP => 0x5,
            Self::NOT_LEADER => 0x6,
            Self::PLAYER_WRONG_FACTION => 0x7,
            Self::IGNORING_YOU => 0x8,
        }
    }

    pub const fn new() -> Self {
        Self::SUCCESS
    }

}

impl ConstantSized for PartyResult {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for PartyResult {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for PartyResult {
    fn default() -> Self {
        Self::SUCCESS
    }
}

impl std::fmt::Display for PartyResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SUCCESS => f.write_str("SUCCESS"),
            Self::BAD_PLAYER_NAME => f.write_str("BAD_PLAYER_NAME"),
            Self::TARGET_NOT_IN_GROUP => f.write_str("TARGET_NOT_IN_GROUP"),
            Self::GROUP_FULL => f.write_str("GROUP_FULL"),
            Self::ALREADY_IN_GROUP => f.write_str("ALREADY_IN_GROUP"),
            Self::NOT_IN_GROUP => f.write_str("NOT_IN_GROUP"),
            Self::NOT_LEADER => f.write_str("NOT_LEADER"),
            Self::PLAYER_WRONG_FACTION => f.write_str("PLAYER_WRONG_FACTION"),
            Self::IGNORING_YOU => f.write_str("IGNORING_YOU"),
        }
    }
}

impl TryFrom<u8> for PartyResult {
    type Error = TryFromPartyResultError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SUCCESS),
            1 => Ok(Self::BAD_PLAYER_NAME),
            2 => Ok(Self::TARGET_NOT_IN_GROUP),
            3 => Ok(Self::GROUP_FULL),
            4 => Ok(Self::ALREADY_IN_GROUP),
            5 => Ok(Self::NOT_IN_GROUP),
            6 => Ok(Self::NOT_LEADER),
            7 => Ok(Self::PLAYER_WRONG_FACTION),
            8 => Ok(Self::IGNORING_YOU),
            _ => Err(TryFromPartyResultError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromPartyResultError {
    value: u8,
}

impl TryFromPartyResultError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum PartyResultError {
    Read(std::io::Error),
    TryFrom(TryFromPartyResultError),
}

impl std::error::Error for PartyResultError {}
impl std::fmt::Display for TryFromPartyResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'PartyResult': '{}'", self.value))
    }
}

impl std::fmt::Display for PartyResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for PartyResultError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromPartyResultError> for PartyResultError {
    fn from(value: TryFromPartyResultError) -> Self {
        Self::TryFrom(value)
    }
}

