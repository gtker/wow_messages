use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new_all.wowm:2322`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new_all.wowm#L2322):
/// ```text
/// enum TrainingFailureReason : u32 {
///     UNAVAILABLE = 0;
///     NOT_ENOUGH_MONEY = 1;
///     NOT_ENOUGH_SKILL = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum TrainingFailureReason {
    UNAVAILABLE,
    NOT_ENOUGH_MONEY,
    NOT_ENOUGH_SKILL,
}

impl ReadableAndWritable for TrainingFailureReason {
    type Error = TrainingFailureReasonError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl TrainingFailureReason {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, TrainingFailureReasonError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, TrainingFailureReasonError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, TrainingFailureReasonError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::UNAVAILABLE => 0x0,
            Self::NOT_ENOUGH_MONEY => 0x1,
            Self::NOT_ENOUGH_SKILL => 0x2,
        }
    }

    pub const fn new() -> Self {
        Self::UNAVAILABLE
    }

}

impl ConstantSized for TrainingFailureReason {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for TrainingFailureReason {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for TrainingFailureReason {
    fn default() -> Self {
        Self::UNAVAILABLE
    }
}

impl std::fmt::Display for TrainingFailureReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UNAVAILABLE => f.write_str("UNAVAILABLE"),
            Self::NOT_ENOUGH_MONEY => f.write_str("NOT_ENOUGH_MONEY"),
            Self::NOT_ENOUGH_SKILL => f.write_str("NOT_ENOUGH_SKILL"),
        }
    }
}

impl TryFrom<u32> for TrainingFailureReason {
    type Error = TryFromTrainingFailureReasonError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::UNAVAILABLE),
            1 => Ok(Self::NOT_ENOUGH_MONEY),
            2 => Ok(Self::NOT_ENOUGH_SKILL),
            _ => Err(TryFromTrainingFailureReasonError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromTrainingFailureReasonError {
    value: u32,
}

impl TryFromTrainingFailureReasonError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum TrainingFailureReasonError {
    Read(std::io::Error),
    TryFrom(TryFromTrainingFailureReasonError),
}

impl std::error::Error for TrainingFailureReasonError {}
impl std::fmt::Display for TryFromTrainingFailureReasonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'TrainingFailureReason': '{}'", self.value))
    }
}

impl std::fmt::Display for TrainingFailureReasonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for TrainingFailureReasonError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromTrainingFailureReasonError> for TrainingFailureReasonError {
    fn from(value: TryFromTrainingFailureReasonError) -> Self {
        Self::TryFrom(value)
    }
}

