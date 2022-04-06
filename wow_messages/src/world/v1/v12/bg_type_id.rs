use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/smsg_group_joined_battleground.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/smsg_group_joined_battleground.wowm#L3):
/// ```text
/// enum BgTypeId : u32 {
///     NOT_ELIGIBLE = 0;
///     QUEUED_FOR_AV = 1;
///     QUEUED_FOR_WSG = 2;
///     QUEUED_FOR_AB = 3;
///     REMOVE_FROM_QUEUE = 0xFFFFFFFE;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum BgTypeId {
    NOT_ELIGIBLE,
    QUEUED_FOR_AV,
    QUEUED_FOR_WSG,
    QUEUED_FOR_AB,
    REMOVE_FROM_QUEUE,
}

impl ReadableAndWritable for BgTypeId {
    type Error = BgTypeIdError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl BgTypeId {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BgTypeIdError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BgTypeIdError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BgTypeIdError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::NOT_ELIGIBLE => 0x0,
            Self::QUEUED_FOR_AV => 0x1,
            Self::QUEUED_FOR_WSG => 0x2,
            Self::QUEUED_FOR_AB => 0x3,
            Self::REMOVE_FROM_QUEUE => 0xfffffffe,
        }
    }

    pub const fn new() -> Self {
        Self::NOT_ELIGIBLE
    }

}

impl ConstantSized for BgTypeId {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for BgTypeId {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for BgTypeId {
    fn default() -> Self {
        Self::NOT_ELIGIBLE
    }
}

impl std::fmt::Display for BgTypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NOT_ELIGIBLE => f.write_str("NOT_ELIGIBLE"),
            Self::QUEUED_FOR_AV => f.write_str("QUEUED_FOR_AV"),
            Self::QUEUED_FOR_WSG => f.write_str("QUEUED_FOR_WSG"),
            Self::QUEUED_FOR_AB => f.write_str("QUEUED_FOR_AB"),
            Self::REMOVE_FROM_QUEUE => f.write_str("REMOVE_FROM_QUEUE"),
        }
    }
}

impl TryFrom<u32> for BgTypeId {
    type Error = TryFromBgTypeIdError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NOT_ELIGIBLE),
            1 => Ok(Self::QUEUED_FOR_AV),
            2 => Ok(Self::QUEUED_FOR_WSG),
            3 => Ok(Self::QUEUED_FOR_AB),
            4294967294 => Ok(Self::REMOVE_FROM_QUEUE),
            _ => Err(TryFromBgTypeIdError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromBgTypeIdError {
    value: u32,
}

impl TryFromBgTypeIdError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum BgTypeIdError {
    Read(std::io::Error),
    TryFrom(TryFromBgTypeIdError),
}

impl std::error::Error for BgTypeIdError {}
impl std::fmt::Display for TryFromBgTypeIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'BgTypeId': '{}'", self.value))
    }
}

impl std::fmt::Display for BgTypeIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for BgTypeIdError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromBgTypeIdError> for BgTypeIdError {
    fn from(value: TryFromBgTypeIdError) -> Self {
        Self::TryFrom(value)
    }
}

