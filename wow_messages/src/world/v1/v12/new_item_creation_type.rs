use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:2697`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L2697):
/// ```text
/// enum NewItemCreationType : u32 {
///     RECEIVED = 0;
///     CREATED = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum NewItemCreationType {
    RECEIVED,
    CREATED,
}

impl ReadableAndWritable for NewItemCreationType {
    type Error = NewItemCreationTypeError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl NewItemCreationType {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, NewItemCreationTypeError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, NewItemCreationTypeError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, NewItemCreationTypeError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::RECEIVED => 0x0,
            Self::CREATED => 0x1,
        }
    }

    pub const fn new() -> Self {
        Self::RECEIVED
    }

}

impl ConstantSized for NewItemCreationType {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for NewItemCreationType {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for NewItemCreationType {
    fn default() -> Self {
        Self::RECEIVED
    }
}

impl std::fmt::Display for NewItemCreationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RECEIVED => f.write_str("RECEIVED"),
            Self::CREATED => f.write_str("CREATED"),
        }
    }
}

impl TryFrom<u32> for NewItemCreationType {
    type Error = TryFromNewItemCreationTypeError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::RECEIVED),
            1 => Ok(Self::CREATED),
            _ => Err(TryFromNewItemCreationTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromNewItemCreationTypeError {
    value: u32,
}

impl TryFromNewItemCreationTypeError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum NewItemCreationTypeError {
    Read(std::io::Error),
    TryFrom(TryFromNewItemCreationTypeError),
}

impl std::error::Error for NewItemCreationTypeError {}
impl std::fmt::Display for TryFromNewItemCreationTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'NewItemCreationType': '{}'", self.value))
    }
}

impl std::fmt::Display for NewItemCreationTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for NewItemCreationTypeError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromNewItemCreationTypeError> for NewItemCreationTypeError {
    fn from(value: TryFromNewItemCreationTypeError) -> Self {
        Self::TryFrom(value)
    }
}

