use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new_all.wowm:2199`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new_all.wowm#L2199):
/// ```text
/// enum PetCommandState : u8 {
///     STAY = 0;
///     FOLLOW = 1;
///     ATTACK = 2;
///     DISMISS = 3;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PetCommandState {
    STAY,
    FOLLOW,
    ATTACK,
    DISMISS,
}

impl ReadableAndWritable for PetCommandState {
    type Error = PetCommandStateError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl PetCommandState {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetCommandStateError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetCommandStateError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetCommandStateError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetCommandStateError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetCommandStateError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetCommandStateError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::STAY => 0x0,
            Self::FOLLOW => 0x1,
            Self::ATTACK => 0x2,
            Self::DISMISS => 0x3,
        }
    }

    pub const fn new() -> Self {
        Self::STAY
    }

}

impl ConstantSized for PetCommandState {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for PetCommandState {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for PetCommandState {
    fn default() -> Self {
        Self::STAY
    }
}

impl std::fmt::Display for PetCommandState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::STAY => f.write_str("STAY"),
            Self::FOLLOW => f.write_str("FOLLOW"),
            Self::ATTACK => f.write_str("ATTACK"),
            Self::DISMISS => f.write_str("DISMISS"),
        }
    }
}

impl TryFrom<u8> for PetCommandState {
    type Error = TryFromPetCommandStateError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::STAY),
            1 => Ok(Self::FOLLOW),
            2 => Ok(Self::ATTACK),
            3 => Ok(Self::DISMISS),
            _ => Err(TryFromPetCommandStateError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromPetCommandStateError {
    value: u8,
}

impl TryFromPetCommandStateError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum PetCommandStateError {
    Read(std::io::Error),
    TryFrom(TryFromPetCommandStateError),
}

impl std::error::Error for PetCommandStateError {}
impl std::fmt::Display for TryFromPetCommandStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'PetCommandState': '{}'", self.value))
    }
}

impl std::fmt::Display for PetCommandStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for PetCommandStateError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromPetCommandStateError> for PetCommandStateError {
    fn from(value: TryFromPetCommandStateError) -> Self {
        Self::TryFrom(value)
    }
}

