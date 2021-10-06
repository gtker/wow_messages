use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/new4.wowm:426`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/new4.wowm):
/// ```text
/// enum RaidInstanceMessage : u32 {
///     WARNING_HOURS = 1;
///     WARNING_MIN = 2;
///     WARNING_MIN_SOON = 3;
///     WELCOME = 4;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum RaidInstanceMessage {
    WARNING_HOURS,
    WARNING_MIN,
    WARNING_MIN_SOON,
    WELCOME,
}

impl ReadableAndWritable for RaidInstanceMessage {
    type Error = RaidInstanceMessageError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl RaidInstanceMessage {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RaidInstanceMessageError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RaidInstanceMessageError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RaidInstanceMessageError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::WARNING_HOURS => 0x1,
            Self::WARNING_MIN => 0x2,
            Self::WARNING_MIN_SOON => 0x3,
            Self::WELCOME => 0x4,
        }
    }

    pub const fn new() -> Self {
        Self::WARNING_HOURS
    }

}

impl ConstantSized for RaidInstanceMessage {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for RaidInstanceMessage {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for RaidInstanceMessage {
    fn default() -> Self {
        Self::WARNING_HOURS
    }
}

impl std::fmt::Display for RaidInstanceMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WARNING_HOURS => f.write_str("WARNING_HOURS"),
            Self::WARNING_MIN => f.write_str("WARNING_MIN"),
            Self::WARNING_MIN_SOON => f.write_str("WARNING_MIN_SOON"),
            Self::WELCOME => f.write_str("WELCOME"),
        }
    }
}

impl TryFrom<u32> for RaidInstanceMessage {
    type Error = TryFromRaidInstanceMessageError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::WARNING_HOURS),
            2 => Ok(Self::WARNING_MIN),
            3 => Ok(Self::WARNING_MIN_SOON),
            4 => Ok(Self::WELCOME),
            _ => Err(TryFromRaidInstanceMessageError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromRaidInstanceMessageError {
    value: u32,
}

impl TryFromRaidInstanceMessageError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum RaidInstanceMessageError {
    Read(std::io::Error),
    TryFrom(TryFromRaidInstanceMessageError),
}

impl std::error::Error for RaidInstanceMessageError {}
impl std::fmt::Display for TryFromRaidInstanceMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'RaidInstanceMessage': '{}'", self.value))
    }
}

impl std::fmt::Display for RaidInstanceMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for RaidInstanceMessageError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromRaidInstanceMessageError> for RaidInstanceMessageError {
    fn from(value: TryFromRaidInstanceMessageError) -> Self {
        Self::TryFrom(value)
    }
}

