use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:461`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L461):
/// ```text
/// enum PetReactState : u8 {
///     PASSIVE = 0;
///     DEFENSIVE = 1;
///     AGGRESSIVE = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PetReactState {
    PASSIVE,
    DEFENSIVE,
    AGGRESSIVE,
}

impl ReadableAndWritable for PetReactState {
    type Error = PetReactStateError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl PetReactState {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetReactStateError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetReactStateError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetReactStateError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetReactStateError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetReactStateError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetReactStateError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::PASSIVE => 0x0,
            Self::DEFENSIVE => 0x1,
            Self::AGGRESSIVE => 0x2,
        }
    }

    pub const fn new() -> Self {
        Self::PASSIVE
    }

}

impl ConstantSized for PetReactState {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for PetReactState {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for PetReactState {
    fn default() -> Self {
        Self::PASSIVE
    }
}

impl std::fmt::Display for PetReactState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PASSIVE => f.write_str("PASSIVE"),
            Self::DEFENSIVE => f.write_str("DEFENSIVE"),
            Self::AGGRESSIVE => f.write_str("AGGRESSIVE"),
        }
    }
}

impl TryFrom<u8> for PetReactState {
    type Error = TryFromPetReactStateError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PASSIVE),
            1 => Ok(Self::DEFENSIVE),
            2 => Ok(Self::AGGRESSIVE),
            _ => Err(TryFromPetReactStateError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromPetReactStateError {
    value: u8,
}

impl TryFromPetReactStateError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum PetReactStateError {
    Read(std::io::Error),
    TryFrom(TryFromPetReactStateError),
}

impl std::error::Error for PetReactStateError {}
impl std::fmt::Display for TryFromPetReactStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'PetReactState': '{}'", self.value))
    }
}

impl std::fmt::Display for PetReactStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for PetReactStateError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromPetReactStateError> for PetReactStateError {
    fn from(value: TryFromPetReactStateError) -> Self {
        Self::TryFrom(value)
    }
}

