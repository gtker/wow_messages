use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PetTalkReason {
    SPECIAL_SPELL,
    ATTACK,
}

impl ReadableAndWritable for PetTalkReason {
    type Error = PetTalkReasonError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl PetTalkReason {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetTalkReasonError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetTalkReasonError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetTalkReasonError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::SPECIAL_SPELL => 0x0,
            Self::ATTACK => 0x1,
        }
    }

    pub const fn new() -> Self {
        Self::SPECIAL_SPELL
    }

}

impl ConstantSized for PetTalkReason {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for PetTalkReason {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for PetTalkReason {
    fn default() -> Self {
        Self::SPECIAL_SPELL
    }
}

impl std::fmt::Display for PetTalkReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SPECIAL_SPELL => f.write_str("SPECIAL_SPELL"),
            Self::ATTACK => f.write_str("ATTACK"),
        }
    }
}

impl TryFrom<u32> for PetTalkReason {
    type Error = TryFromPetTalkReasonError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SPECIAL_SPELL),
            1 => Ok(Self::ATTACK),
            _ => Err(TryFromPetTalkReasonError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromPetTalkReasonError {
    value: u32,
}

impl TryFromPetTalkReasonError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum PetTalkReasonError {
    Read(std::io::Error),
    TryFrom(TryFromPetTalkReasonError),
}

impl std::error::Error for PetTalkReasonError {}
impl std::fmt::Display for TryFromPetTalkReasonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'PetTalkReason': '{}'", self.value))
    }
}

impl std::fmt::Display for PetTalkReasonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for PetTalkReasonError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromPetTalkReasonError> for PetTalkReasonError {
    fn from(value: TryFromPetTalkReasonError) -> Self {
        Self::TryFrom(value)
    }
}

