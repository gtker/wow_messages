use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:3600`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L3600):
/// ```text
/// enum TimerType : u32 {
///     FATIGUE = 0;
///     BREATH = 1;
///     FEIGNDEATH = 2;
///     ENVIRONMENTAL = 3;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum TimerType {
    FATIGUE,
    BREATH,
    FEIGNDEATH,
    ENVIRONMENTAL,
}

impl ReadableAndWritable for TimerType {
    type Error = TimerTypeError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl TimerType {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, TimerTypeError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, TimerTypeError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, TimerTypeError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::FATIGUE => 0x0,
            Self::BREATH => 0x1,
            Self::FEIGNDEATH => 0x2,
            Self::ENVIRONMENTAL => 0x3,
        }
    }

    pub const fn new() -> Self {
        Self::FATIGUE
    }

}

impl ConstantSized for TimerType {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for TimerType {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for TimerType {
    fn default() -> Self {
        Self::FATIGUE
    }
}

impl std::fmt::Display for TimerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FATIGUE => f.write_str("FATIGUE"),
            Self::BREATH => f.write_str("BREATH"),
            Self::FEIGNDEATH => f.write_str("FEIGNDEATH"),
            Self::ENVIRONMENTAL => f.write_str("ENVIRONMENTAL"),
        }
    }
}

impl TryFrom<u32> for TimerType {
    type Error = TryFromTimerTypeError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::FATIGUE),
            1 => Ok(Self::BREATH),
            2 => Ok(Self::FEIGNDEATH),
            3 => Ok(Self::ENVIRONMENTAL),
            _ => Err(TryFromTimerTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromTimerTypeError {
    value: u32,
}

impl TryFromTimerTypeError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum TimerTypeError {
    Read(std::io::Error),
    TryFrom(TryFromTimerTypeError),
}

impl std::error::Error for TimerTypeError {}
impl std::fmt::Display for TryFromTimerTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'TimerType': '{}'", self.value))
    }
}

impl std::fmt::Display for TimerTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for TimerTypeError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromTimerTypeError> for TimerTypeError {
    fn from(value: TryFromTimerTypeError) -> Self {
        Self::TryFrom(value)
    }
}

