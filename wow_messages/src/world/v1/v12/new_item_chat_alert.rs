use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:4737`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L4737):
/// ```text
/// enum NewItemChatAlert : u32 {
///     DO_NOT_SHOW = 0;
///     SHOW = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum NewItemChatAlert {
    DO_NOT_SHOW,
    SHOW,
}

impl ReadableAndWritable for NewItemChatAlert {
    type Error = NewItemChatAlertError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl NewItemChatAlert {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, NewItemChatAlertError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, NewItemChatAlertError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, NewItemChatAlertError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::DO_NOT_SHOW => 0x0,
            Self::SHOW => 0x1,
        }
    }

    pub const fn new() -> Self {
        Self::DO_NOT_SHOW
    }

}

impl ConstantSized for NewItemChatAlert {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for NewItemChatAlert {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for NewItemChatAlert {
    fn default() -> Self {
        Self::DO_NOT_SHOW
    }
}

impl std::fmt::Display for NewItemChatAlert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DO_NOT_SHOW => f.write_str("DO_NOT_SHOW"),
            Self::SHOW => f.write_str("SHOW"),
        }
    }
}

impl TryFrom<u32> for NewItemChatAlert {
    type Error = TryFromNewItemChatAlertError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DO_NOT_SHOW),
            1 => Ok(Self::SHOW),
            _ => Err(TryFromNewItemChatAlertError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromNewItemChatAlertError {
    value: u32,
}

impl TryFromNewItemChatAlertError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum NewItemChatAlertError {
    Read(std::io::Error),
    TryFrom(TryFromNewItemChatAlertError),
}

impl std::error::Error for NewItemChatAlertError {}
impl std::fmt::Display for TryFromNewItemChatAlertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'NewItemChatAlert': '{}'", self.value))
    }
}

impl std::fmt::Display for NewItemChatAlertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for NewItemChatAlertError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromNewItemChatAlertError> for NewItemChatAlertError {
    fn from(value: TryFromNewItemChatAlertError) -> Self {
        Self::TryFrom(value)
    }
}

