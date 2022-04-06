use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/remaining.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/remaining.wowm#L3):
/// ```text
/// enum UnitStandState : u8 {
///     STAND = 0;
///     SIT = 1;
///     SIT_CHAIR = 2;
///     SLEEP = 3;
///     SIT_LOW_CHAIR = 4;
///     SIT_MEDIUM_CHAIR = 5;
///     SIT_HIGH_CHAIR = 6;
///     DEAD = 7;
///     KNEEL = 8;
///     CUSTOM = 9;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum UnitStandState {
    STAND,
    SIT,
    SIT_CHAIR,
    SLEEP,
    SIT_LOW_CHAIR,
    SIT_MEDIUM_CHAIR,
    SIT_HIGH_CHAIR,
    DEAD,
    KNEEL,
    CUSTOM,
}

impl ReadableAndWritable for UnitStandState {
    type Error = UnitStandStateError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl UnitStandState {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, UnitStandStateError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, UnitStandStateError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, UnitStandStateError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, UnitStandStateError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, UnitStandStateError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, UnitStandStateError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::STAND => 0x0,
            Self::SIT => 0x1,
            Self::SIT_CHAIR => 0x2,
            Self::SLEEP => 0x3,
            Self::SIT_LOW_CHAIR => 0x4,
            Self::SIT_MEDIUM_CHAIR => 0x5,
            Self::SIT_HIGH_CHAIR => 0x6,
            Self::DEAD => 0x7,
            Self::KNEEL => 0x8,
            Self::CUSTOM => 0x9,
        }
    }

    pub const fn new() -> Self {
        Self::STAND
    }

}

impl ConstantSized for UnitStandState {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for UnitStandState {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for UnitStandState {
    fn default() -> Self {
        Self::STAND
    }
}

impl std::fmt::Display for UnitStandState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::STAND => f.write_str("STAND"),
            Self::SIT => f.write_str("SIT"),
            Self::SIT_CHAIR => f.write_str("SIT_CHAIR"),
            Self::SLEEP => f.write_str("SLEEP"),
            Self::SIT_LOW_CHAIR => f.write_str("SIT_LOW_CHAIR"),
            Self::SIT_MEDIUM_CHAIR => f.write_str("SIT_MEDIUM_CHAIR"),
            Self::SIT_HIGH_CHAIR => f.write_str("SIT_HIGH_CHAIR"),
            Self::DEAD => f.write_str("DEAD"),
            Self::KNEEL => f.write_str("KNEEL"),
            Self::CUSTOM => f.write_str("CUSTOM"),
        }
    }
}

impl TryFrom<u8> for UnitStandState {
    type Error = TryFromUnitStandStateError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::STAND),
            1 => Ok(Self::SIT),
            2 => Ok(Self::SIT_CHAIR),
            3 => Ok(Self::SLEEP),
            4 => Ok(Self::SIT_LOW_CHAIR),
            5 => Ok(Self::SIT_MEDIUM_CHAIR),
            6 => Ok(Self::SIT_HIGH_CHAIR),
            7 => Ok(Self::DEAD),
            8 => Ok(Self::KNEEL),
            9 => Ok(Self::CUSTOM),
            _ => Err(TryFromUnitStandStateError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromUnitStandStateError {
    value: u8,
}

impl TryFromUnitStandStateError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum UnitStandStateError {
    Read(std::io::Error),
    TryFrom(TryFromUnitStandStateError),
}

impl std::error::Error for UnitStandStateError {}
impl std::fmt::Display for TryFromUnitStandStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'UnitStandState': '{}'", self.value))
    }
}

impl std::fmt::Display for UnitStandStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for UnitStandStateError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromUnitStandStateError> for UnitStandStateError {
    fn from(value: TryFromUnitStandStateError) -> Self {
        Self::TryFrom(value)
    }
}

