use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/add_messages.wowm:83`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/add_messages.wowm#L83):
/// ```text
/// enum LogoutResult : u32 {
///     SUCCESS = 0;
///     FAILURE_IN_COMBAT = 1;
///     FAILURE_FROZEN_BY_GM = 2;
///     FAILURE_JUMPING_OR_FALLING = 3;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum LogoutResult {
    SUCCESS,
    FAILURE_IN_COMBAT,
    FAILURE_FROZEN_BY_GM,
    FAILURE_JUMPING_OR_FALLING,
}

impl ReadableAndWritable for LogoutResult {
    type Error = LogoutResultError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl LogoutResult {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, LogoutResultError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, LogoutResultError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, LogoutResultError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::SUCCESS => 0x0,
            Self::FAILURE_IN_COMBAT => 0x1,
            Self::FAILURE_FROZEN_BY_GM => 0x2,
            Self::FAILURE_JUMPING_OR_FALLING => 0x3,
        }
    }

    pub const fn new() -> Self {
        Self::SUCCESS
    }

}

impl ConstantSized for LogoutResult {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for LogoutResult {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for LogoutResult {
    fn default() -> Self {
        Self::SUCCESS
    }
}

impl std::fmt::Display for LogoutResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SUCCESS => f.write_str("SUCCESS"),
            Self::FAILURE_IN_COMBAT => f.write_str("FAILURE_IN_COMBAT"),
            Self::FAILURE_FROZEN_BY_GM => f.write_str("FAILURE_FROZEN_BY_GM"),
            Self::FAILURE_JUMPING_OR_FALLING => f.write_str("FAILURE_JUMPING_OR_FALLING"),
        }
    }
}

impl TryFrom<u32> for LogoutResult {
    type Error = TryFromLogoutResultError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SUCCESS),
            1 => Ok(Self::FAILURE_IN_COMBAT),
            2 => Ok(Self::FAILURE_FROZEN_BY_GM),
            3 => Ok(Self::FAILURE_JUMPING_OR_FALLING),
            _ => Err(TryFromLogoutResultError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromLogoutResultError {
    value: u32,
}

impl TryFromLogoutResultError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum LogoutResultError {
    Read(std::io::Error),
    TryFrom(TryFromLogoutResultError),
}

impl std::error::Error for LogoutResultError {}
impl std::fmt::Display for TryFromLogoutResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'LogoutResult': '{}'", self.value))
    }
}

impl std::fmt::Display for LogoutResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for LogoutResultError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromLogoutResultError> for LogoutResultError {
    fn from(value: TryFromLogoutResultError) -> Self {
        Self::TryFrom(value)
    }
}

