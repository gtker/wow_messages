use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new4.wowm:378`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new4.wowm#L378):
/// ```text
/// enum WeatherChangeType : u8 {
///     SMOOTH = 0;
///     INSTANT = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum WeatherChangeType {
    SMOOTH,
    INSTANT,
}

impl ReadableAndWritable for WeatherChangeType {
    type Error = WeatherChangeTypeError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl WeatherChangeType {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, WeatherChangeTypeError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, WeatherChangeTypeError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, WeatherChangeTypeError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, WeatherChangeTypeError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, WeatherChangeTypeError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, WeatherChangeTypeError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::SMOOTH => 0x0,
            Self::INSTANT => 0x1,
        }
    }

    pub const fn new() -> Self {
        Self::SMOOTH
    }

}

impl ConstantSized for WeatherChangeType {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for WeatherChangeType {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for WeatherChangeType {
    fn default() -> Self {
        Self::SMOOTH
    }
}

impl std::fmt::Display for WeatherChangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SMOOTH => f.write_str("SMOOTH"),
            Self::INSTANT => f.write_str("INSTANT"),
        }
    }
}

impl TryFrom<u8> for WeatherChangeType {
    type Error = TryFromWeatherChangeTypeError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SMOOTH),
            1 => Ok(Self::INSTANT),
            _ => Err(TryFromWeatherChangeTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromWeatherChangeTypeError {
    value: u8,
}

impl TryFromWeatherChangeTypeError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum WeatherChangeTypeError {
    Read(std::io::Error),
    TryFrom(TryFromWeatherChangeTypeError),
}

impl std::error::Error for WeatherChangeTypeError {}
impl std::fmt::Display for TryFromWeatherChangeTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'WeatherChangeType': '{}'", self.value))
    }
}

impl std::fmt::Display for WeatherChangeTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for WeatherChangeTypeError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromWeatherChangeTypeError> for WeatherChangeTypeError {
    fn from(value: TryFromWeatherChangeTypeError) -> Self {
        Self::TryFrom(value)
    }
}

