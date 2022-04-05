use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new4.wowm:477`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new4.wowm#L477):
/// ```text
/// enum InstanceResetFailedReason : u8 {
///     GENERAL = 0;
///     OFFLINE = 1;
///     ZONING = 2;
///     SILENTLY = 3;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum InstanceResetFailedReason {
    GENERAL,
    OFFLINE,
    ZONING,
    SILENTLY,
}

impl ReadableAndWritable for InstanceResetFailedReason {
    type Error = InstanceResetFailedReasonError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl InstanceResetFailedReason {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, InstanceResetFailedReasonError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, InstanceResetFailedReasonError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, InstanceResetFailedReasonError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, InstanceResetFailedReasonError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, InstanceResetFailedReasonError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, InstanceResetFailedReasonError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::GENERAL => 0x0,
            Self::OFFLINE => 0x1,
            Self::ZONING => 0x2,
            Self::SILENTLY => 0x3,
        }
    }

    pub const fn new() -> Self {
        Self::GENERAL
    }

}

impl ConstantSized for InstanceResetFailedReason {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for InstanceResetFailedReason {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for InstanceResetFailedReason {
    fn default() -> Self {
        Self::GENERAL
    }
}

impl std::fmt::Display for InstanceResetFailedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GENERAL => f.write_str("GENERAL"),
            Self::OFFLINE => f.write_str("OFFLINE"),
            Self::ZONING => f.write_str("ZONING"),
            Self::SILENTLY => f.write_str("SILENTLY"),
        }
    }
}

impl TryFrom<u8> for InstanceResetFailedReason {
    type Error = TryFromInstanceResetFailedReasonError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::GENERAL),
            1 => Ok(Self::OFFLINE),
            2 => Ok(Self::ZONING),
            3 => Ok(Self::SILENTLY),
            _ => Err(TryFromInstanceResetFailedReasonError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromInstanceResetFailedReasonError {
    value: u8,
}

impl TryFromInstanceResetFailedReasonError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum InstanceResetFailedReasonError {
    Read(std::io::Error),
    TryFrom(TryFromInstanceResetFailedReasonError),
}

impl std::error::Error for InstanceResetFailedReasonError {}
impl std::fmt::Display for TryFromInstanceResetFailedReasonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'InstanceResetFailedReason': '{}'", self.value))
    }
}

impl std::fmt::Display for InstanceResetFailedReasonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for InstanceResetFailedReasonError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromInstanceResetFailedReasonError> for InstanceResetFailedReasonError {
    fn from(value: TryFromInstanceResetFailedReasonError) -> Self {
        Self::TryFrom(value)
    }
}

