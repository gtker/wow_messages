use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new_all.wowm:2716`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new_all.wowm#L2716):
/// ```text
/// enum MailAction : u32 {
///     SEND = 0;
///     MONEY_TAKEN = 1;
///     ITEM_TAKEN = 2;
///     RETURNED_TO_SENDER = 3;
///     DELETED = 4;
///     MADE_PERMANENT = 5;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum MailAction {
    SEND,
    MONEY_TAKEN,
    ITEM_TAKEN,
    RETURNED_TO_SENDER,
    DELETED,
    MADE_PERMANENT,
}

impl ReadableAndWritable for MailAction {
    type Error = MailActionError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl MailAction {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MailActionError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MailActionError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MailActionError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::SEND => 0x0,
            Self::MONEY_TAKEN => 0x1,
            Self::ITEM_TAKEN => 0x2,
            Self::RETURNED_TO_SENDER => 0x3,
            Self::DELETED => 0x4,
            Self::MADE_PERMANENT => 0x5,
        }
    }

    pub const fn new() -> Self {
        Self::SEND
    }

}

impl ConstantSized for MailAction {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for MailAction {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for MailAction {
    fn default() -> Self {
        Self::SEND
    }
}

impl std::fmt::Display for MailAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SEND => f.write_str("SEND"),
            Self::MONEY_TAKEN => f.write_str("MONEY_TAKEN"),
            Self::ITEM_TAKEN => f.write_str("ITEM_TAKEN"),
            Self::RETURNED_TO_SENDER => f.write_str("RETURNED_TO_SENDER"),
            Self::DELETED => f.write_str("DELETED"),
            Self::MADE_PERMANENT => f.write_str("MADE_PERMANENT"),
        }
    }
}

impl TryFrom<u32> for MailAction {
    type Error = TryFromMailActionError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SEND),
            1 => Ok(Self::MONEY_TAKEN),
            2 => Ok(Self::ITEM_TAKEN),
            3 => Ok(Self::RETURNED_TO_SENDER),
            4 => Ok(Self::DELETED),
            5 => Ok(Self::MADE_PERMANENT),
            _ => Err(TryFromMailActionError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromMailActionError {
    value: u32,
}

impl TryFromMailActionError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum MailActionError {
    Read(std::io::Error),
    TryFrom(TryFromMailActionError),
}

impl std::error::Error for MailActionError {}
impl std::fmt::Display for TryFromMailActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'MailAction': '{}'", self.value))
    }
}

impl std::fmt::Display for MailActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for MailActionError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromMailActionError> for MailActionError {
    fn from(value: TryFromMailActionError) -> Self {
        Self::TryFrom(value)
    }
}

