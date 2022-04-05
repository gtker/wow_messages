use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:3961`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L3961):
/// ```text
/// enum RaidGroupError : u32 {
///     REQUIRED = 1;
///     FULL = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum RaidGroupError {
    REQUIRED,
    FULL,
}

impl ReadableAndWritable for RaidGroupError {
    type Error = RaidGroupErrorError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl RaidGroupError {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RaidGroupErrorError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RaidGroupErrorError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RaidGroupErrorError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::REQUIRED => 0x1,
            Self::FULL => 0x2,
        }
    }

    pub const fn new() -> Self {
        Self::REQUIRED
    }

}

impl ConstantSized for RaidGroupError {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for RaidGroupError {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for RaidGroupError {
    fn default() -> Self {
        Self::REQUIRED
    }
}

impl std::fmt::Display for RaidGroupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::REQUIRED => f.write_str("REQUIRED"),
            Self::FULL => f.write_str("FULL"),
        }
    }
}

impl TryFrom<u32> for RaidGroupError {
    type Error = TryFromRaidGroupErrorError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::REQUIRED),
            2 => Ok(Self::FULL),
            _ => Err(TryFromRaidGroupErrorError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromRaidGroupErrorError {
    value: u32,
}

impl TryFromRaidGroupErrorError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum RaidGroupErrorError {
    Read(std::io::Error),
    TryFrom(TryFromRaidGroupErrorError),
}

impl std::error::Error for RaidGroupErrorError {}
impl std::fmt::Display for TryFromRaidGroupErrorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'RaidGroupError': '{}'", self.value))
    }
}

impl std::fmt::Display for RaidGroupErrorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for RaidGroupErrorError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromRaidGroupErrorError> for RaidGroupErrorError {
    fn from(value: TryFromRaidGroupErrorError) -> Self {
        Self::TryFrom(value)
    }
}

