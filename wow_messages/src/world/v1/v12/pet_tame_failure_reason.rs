use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_tame_failure.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_tame_failure.wowm#L3):
/// ```text
/// enum PetTameFailureReason : u8 {
///     INVALIDCREATURE = 1;
///     TOOMANY = 2;
///     CREATUREALREADYOWNED = 3;
///     NOTTAMEABLE = 4;
///     ANOTHERSUMMONACTIVE = 5;
///     UNITSCANTTAME = 6;
///     NOPETAVAILABLE = 7;
///     INTERNALERROR = 8;
///     TOOHIGHLEVEL = 9;
///     DEAD = 10;
///     NOTDEAD = 11;
///     UNKNOWNERROR = 12;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PetTameFailureReason {
    INVALIDCREATURE,
    TOOMANY,
    CREATUREALREADYOWNED,
    NOTTAMEABLE,
    ANOTHERSUMMONACTIVE,
    UNITSCANTTAME,
    NOPETAVAILABLE,
    INTERNALERROR,
    TOOHIGHLEVEL,
    DEAD,
    NOTDEAD,
    UNKNOWNERROR,
}

impl ReadableAndWritable for PetTameFailureReason {
    type Error = PetTameFailureReasonError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl PetTameFailureReason {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetTameFailureReasonError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetTameFailureReasonError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetTameFailureReasonError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetTameFailureReasonError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetTameFailureReasonError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetTameFailureReasonError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::INVALIDCREATURE => 0x1,
            Self::TOOMANY => 0x2,
            Self::CREATUREALREADYOWNED => 0x3,
            Self::NOTTAMEABLE => 0x4,
            Self::ANOTHERSUMMONACTIVE => 0x5,
            Self::UNITSCANTTAME => 0x6,
            Self::NOPETAVAILABLE => 0x7,
            Self::INTERNALERROR => 0x8,
            Self::TOOHIGHLEVEL => 0x9,
            Self::DEAD => 0xa,
            Self::NOTDEAD => 0xb,
            Self::UNKNOWNERROR => 0xc,
        }
    }

    pub const fn new() -> Self {
        Self::INVALIDCREATURE
    }

}

impl ConstantSized for PetTameFailureReason {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for PetTameFailureReason {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for PetTameFailureReason {
    fn default() -> Self {
        Self::INVALIDCREATURE
    }
}

impl std::fmt::Display for PetTameFailureReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::INVALIDCREATURE => f.write_str("INVALIDCREATURE"),
            Self::TOOMANY => f.write_str("TOOMANY"),
            Self::CREATUREALREADYOWNED => f.write_str("CREATUREALREADYOWNED"),
            Self::NOTTAMEABLE => f.write_str("NOTTAMEABLE"),
            Self::ANOTHERSUMMONACTIVE => f.write_str("ANOTHERSUMMONACTIVE"),
            Self::UNITSCANTTAME => f.write_str("UNITSCANTTAME"),
            Self::NOPETAVAILABLE => f.write_str("NOPETAVAILABLE"),
            Self::INTERNALERROR => f.write_str("INTERNALERROR"),
            Self::TOOHIGHLEVEL => f.write_str("TOOHIGHLEVEL"),
            Self::DEAD => f.write_str("DEAD"),
            Self::NOTDEAD => f.write_str("NOTDEAD"),
            Self::UNKNOWNERROR => f.write_str("UNKNOWNERROR"),
        }
    }
}

impl TryFrom<u8> for PetTameFailureReason {
    type Error = TryFromPetTameFailureReasonError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::INVALIDCREATURE),
            2 => Ok(Self::TOOMANY),
            3 => Ok(Self::CREATUREALREADYOWNED),
            4 => Ok(Self::NOTTAMEABLE),
            5 => Ok(Self::ANOTHERSUMMONACTIVE),
            6 => Ok(Self::UNITSCANTTAME),
            7 => Ok(Self::NOPETAVAILABLE),
            8 => Ok(Self::INTERNALERROR),
            9 => Ok(Self::TOOHIGHLEVEL),
            10 => Ok(Self::DEAD),
            11 => Ok(Self::NOTDEAD),
            12 => Ok(Self::UNKNOWNERROR),
            _ => Err(TryFromPetTameFailureReasonError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromPetTameFailureReasonError {
    value: u8,
}

impl TryFromPetTameFailureReasonError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum PetTameFailureReasonError {
    Read(std::io::Error),
    TryFrom(TryFromPetTameFailureReasonError),
}

impl std::error::Error for PetTameFailureReasonError {}
impl std::fmt::Display for TryFromPetTameFailureReasonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'PetTameFailureReason': '{}'", self.value))
    }
}

impl std::fmt::Display for PetTameFailureReasonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for PetTameFailureReasonError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromPetTameFailureReasonError> for PetTameFailureReasonError {
    fn from(value: TryFromPetTameFailureReasonError) -> Self {
        Self::TryFrom(value)
    }
}

