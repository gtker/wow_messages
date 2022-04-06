use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/smsg_spellenergizelog.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/smsg_spellenergizelog.wowm#L3):
/// ```text
/// enum PowerType : u32 {
///     MANA = 0;
///     RAGE = 1;
///     FOCUS = 2;
///     ENERGY = 3;
///     HAPPINESS = 4;
///     HEALTH = 0xFFFFFFFE;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PowerType {
    MANA,
    RAGE,
    FOCUS,
    ENERGY,
    HAPPINESS,
    HEALTH,
}

impl ReadableAndWritable for PowerType {
    type Error = PowerTypeError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl PowerType {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PowerTypeError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PowerTypeError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PowerTypeError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::MANA => 0x0,
            Self::RAGE => 0x1,
            Self::FOCUS => 0x2,
            Self::ENERGY => 0x3,
            Self::HAPPINESS => 0x4,
            Self::HEALTH => 0xfffffffe,
        }
    }

    pub const fn new() -> Self {
        Self::MANA
    }

}

impl ConstantSized for PowerType {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for PowerType {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for PowerType {
    fn default() -> Self {
        Self::MANA
    }
}

impl std::fmt::Display for PowerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MANA => f.write_str("MANA"),
            Self::RAGE => f.write_str("RAGE"),
            Self::FOCUS => f.write_str("FOCUS"),
            Self::ENERGY => f.write_str("ENERGY"),
            Self::HAPPINESS => f.write_str("HAPPINESS"),
            Self::HEALTH => f.write_str("HEALTH"),
        }
    }
}

impl TryFrom<u32> for PowerType {
    type Error = TryFromPowerTypeError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::MANA),
            1 => Ok(Self::RAGE),
            2 => Ok(Self::FOCUS),
            3 => Ok(Self::ENERGY),
            4 => Ok(Self::HAPPINESS),
            4294967294 => Ok(Self::HEALTH),
            _ => Err(TryFromPowerTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromPowerTypeError {
    value: u32,
}

impl TryFromPowerTypeError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum PowerTypeError {
    Read(std::io::Error),
    TryFrom(TryFromPowerTypeError),
}

impl std::error::Error for PowerTypeError {}
impl std::fmt::Display for TryFromPowerTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'PowerType': '{}'", self.value))
    }
}

impl std::fmt::Display for PowerTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for PowerTypeError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromPowerTypeError> for PowerTypeError {
    fn from(value: TryFromPowerTypeError) -> Self {
        Self::TryFrom(value)
    }
}

