use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:3013`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L3013):
/// ```text
/// enum AiReaction : u32 {
///     ALERT = 0;
///     FRIENDLY = 1;
///     HOSTILE = 2;
///     AFRAID = 3;
///     DESTROY = 4;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum AiReaction {
    ALERT,
    FRIENDLY,
    HOSTILE,
    AFRAID,
    DESTROY,
}

impl ReadableAndWritable for AiReaction {
    type Error = AiReactionError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl AiReaction {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, AiReactionError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, AiReactionError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, AiReactionError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::ALERT => 0x0,
            Self::FRIENDLY => 0x1,
            Self::HOSTILE => 0x2,
            Self::AFRAID => 0x3,
            Self::DESTROY => 0x4,
        }
    }

    pub const fn new() -> Self {
        Self::ALERT
    }

}

impl ConstantSized for AiReaction {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for AiReaction {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for AiReaction {
    fn default() -> Self {
        Self::ALERT
    }
}

impl std::fmt::Display for AiReaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ALERT => f.write_str("ALERT"),
            Self::FRIENDLY => f.write_str("FRIENDLY"),
            Self::HOSTILE => f.write_str("HOSTILE"),
            Self::AFRAID => f.write_str("AFRAID"),
            Self::DESTROY => f.write_str("DESTROY"),
        }
    }
}

impl TryFrom<u32> for AiReaction {
    type Error = TryFromAiReactionError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ALERT),
            1 => Ok(Self::FRIENDLY),
            2 => Ok(Self::HOSTILE),
            3 => Ok(Self::AFRAID),
            4 => Ok(Self::DESTROY),
            _ => Err(TryFromAiReactionError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromAiReactionError {
    value: u32,
}

impl TryFromAiReactionError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum AiReactionError {
    Read(std::io::Error),
    TryFrom(TryFromAiReactionError),
}

impl std::error::Error for AiReactionError {}
impl std::fmt::Display for TryFromAiReactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'AiReaction': '{}'", self.value))
    }
}

impl std::fmt::Display for AiReactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for AiReactionError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromAiReactionError> for AiReactionError {
    fn from(value: TryFromAiReactionError) -> Self {
        Self::TryFrom(value)
    }
}

