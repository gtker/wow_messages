use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new5.wowm:384`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new5.wowm#L384):
/// ```text
/// enum NewItemSource : u32 {
///     LOOTED = 0;
///     FROM_NPC = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum NewItemSource {
    LOOTED,
    FROM_NPC,
}

impl ReadableAndWritable for NewItemSource {
    type Error = NewItemSourceError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl NewItemSource {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, NewItemSourceError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, NewItemSourceError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, NewItemSourceError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::LOOTED => 0x0,
            Self::FROM_NPC => 0x1,
        }
    }

    pub const fn new() -> Self {
        Self::LOOTED
    }

}

impl ConstantSized for NewItemSource {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for NewItemSource {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for NewItemSource {
    fn default() -> Self {
        Self::LOOTED
    }
}

impl std::fmt::Display for NewItemSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LOOTED => f.write_str("LOOTED"),
            Self::FROM_NPC => f.write_str("FROM_NPC"),
        }
    }
}

impl TryFrom<u32> for NewItemSource {
    type Error = TryFromNewItemSourceError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::LOOTED),
            1 => Ok(Self::FROM_NPC),
            _ => Err(TryFromNewItemSourceError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromNewItemSourceError {
    value: u32,
}

impl TryFromNewItemSourceError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum NewItemSourceError {
    Read(std::io::Error),
    TryFrom(TryFromNewItemSourceError),
}

impl std::error::Error for NewItemSourceError {}
impl std::fmt::Display for TryFromNewItemSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'NewItemSource': '{}'", self.value))
    }
}

impl std::fmt::Display for NewItemSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for NewItemSourceError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromNewItemSourceError> for NewItemSourceError {
    fn from(value: TryFromNewItemSourceError) -> Self {
        Self::TryFrom(value)
    }
}

