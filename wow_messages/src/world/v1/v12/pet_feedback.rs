use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/new4.wowm:308`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/new4.wowm#L308):
/// ```text
/// enum PetFeedback : u8 {
///     PET_DEAD = 1;
///     NOTHING_TO_EAT = 2;
///     CANT_ATTACK_TARGET = 3;
///     NO_PATH_TO = 4;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PetFeedback {
    PET_DEAD,
    NOTHING_TO_EAT,
    CANT_ATTACK_TARGET,
    NO_PATH_TO,
}

impl ReadableAndWritable for PetFeedback {
    type Error = PetFeedbackError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl PetFeedback {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetFeedbackError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetFeedbackError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetFeedbackError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetFeedbackError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetFeedbackError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetFeedbackError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::PET_DEAD => 0x1,
            Self::NOTHING_TO_EAT => 0x2,
            Self::CANT_ATTACK_TARGET => 0x3,
            Self::NO_PATH_TO => 0x4,
        }
    }

    pub const fn new() -> Self {
        Self::PET_DEAD
    }

}

impl ConstantSized for PetFeedback {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for PetFeedback {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for PetFeedback {
    fn default() -> Self {
        Self::PET_DEAD
    }
}

impl std::fmt::Display for PetFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PET_DEAD => f.write_str("PET_DEAD"),
            Self::NOTHING_TO_EAT => f.write_str("NOTHING_TO_EAT"),
            Self::CANT_ATTACK_TARGET => f.write_str("CANT_ATTACK_TARGET"),
            Self::NO_PATH_TO => f.write_str("NO_PATH_TO"),
        }
    }
}

impl TryFrom<u8> for PetFeedback {
    type Error = TryFromPetFeedbackError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::PET_DEAD),
            2 => Ok(Self::NOTHING_TO_EAT),
            3 => Ok(Self::CANT_ATTACK_TARGET),
            4 => Ok(Self::NO_PATH_TO),
            _ => Err(TryFromPetFeedbackError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromPetFeedbackError {
    value: u8,
}

impl TryFromPetFeedbackError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum PetFeedbackError {
    Read(std::io::Error),
    TryFrom(TryFromPetFeedbackError),
}

impl std::error::Error for PetFeedbackError {}
impl std::fmt::Display for TryFromPetFeedbackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'PetFeedback': '{}'", self.value))
    }
}

impl std::fmt::Display for PetFeedbackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for PetFeedbackError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromPetFeedbackError> for PetFeedbackError {
    fn from(value: TryFromPetFeedbackError) -> Self {
        Self::TryFrom(value)
    }
}

