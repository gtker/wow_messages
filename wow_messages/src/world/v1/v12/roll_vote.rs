use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:415`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L415):
/// ```text
/// enum RollVote : u8 {
///     PASS = 0;
///     NEED = 1;
///     GREED = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum RollVote {
    PASS,
    NEED,
    GREED,
}

impl ReadableAndWritable for RollVote {
    type Error = RollVoteError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl RollVote {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RollVoteError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RollVoteError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RollVoteError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RollVoteError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RollVoteError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RollVoteError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::PASS => 0x0,
            Self::NEED => 0x1,
            Self::GREED => 0x2,
        }
    }

    pub const fn new() -> Self {
        Self::PASS
    }

}

impl ConstantSized for RollVote {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for RollVote {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for RollVote {
    fn default() -> Self {
        Self::PASS
    }
}

impl std::fmt::Display for RollVote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PASS => f.write_str("PASS"),
            Self::NEED => f.write_str("NEED"),
            Self::GREED => f.write_str("GREED"),
        }
    }
}

impl TryFrom<u8> for RollVote {
    type Error = TryFromRollVoteError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PASS),
            1 => Ok(Self::NEED),
            2 => Ok(Self::GREED),
            _ => Err(TryFromRollVoteError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromRollVoteError {
    value: u8,
}

impl TryFromRollVoteError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum RollVoteError {
    Read(std::io::Error),
    TryFrom(TryFromRollVoteError),
}

impl std::error::Error for RollVoteError {}
impl std::fmt::Display for TryFromRollVoteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'RollVote': '{}'", self.value))
    }
}

impl std::fmt::Display for RollVoteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for RollVoteError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromRollVoteError> for RollVoteError {
    fn from(value: TryFromRollVoteError) -> Self {
        Self::TryFrom(value)
    }
}

