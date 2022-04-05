use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new5.wowm:863`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new5.wowm#L863):
/// ```text
/// enum GmTicketType : u8 {
///     STUCK = 1;
///     BEHAVIOR_HARASSMENT = 2;
///     GUILD = 3;
///     ITEM = 4;
///     ENVIRONMENTAL = 5;
///     NONQUEST_CREEP = 6;
///     QUEST_QUESTNPC = 7;
///     TECHNICAL = 8;
///     ACCOUNT_BILLING = 9;
///     CHARACTER = 10;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GmTicketType {
    STUCK,
    BEHAVIOR_HARASSMENT,
    GUILD,
    ITEM,
    ENVIRONMENTAL,
    NONQUEST_CREEP,
    QUEST_QUESTNPC,
    TECHNICAL,
    ACCOUNT_BILLING,
    CHARACTER,
}

impl ReadableAndWritable for GmTicketType {
    type Error = GmTicketTypeError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl GmTicketType {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GmTicketTypeError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GmTicketTypeError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GmTicketTypeError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GmTicketTypeError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GmTicketTypeError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GmTicketTypeError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::STUCK => 0x1,
            Self::BEHAVIOR_HARASSMENT => 0x2,
            Self::GUILD => 0x3,
            Self::ITEM => 0x4,
            Self::ENVIRONMENTAL => 0x5,
            Self::NONQUEST_CREEP => 0x6,
            Self::QUEST_QUESTNPC => 0x7,
            Self::TECHNICAL => 0x8,
            Self::ACCOUNT_BILLING => 0x9,
            Self::CHARACTER => 0xa,
        }
    }

    pub const fn new() -> Self {
        Self::STUCK
    }

}

impl ConstantSized for GmTicketType {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for GmTicketType {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for GmTicketType {
    fn default() -> Self {
        Self::STUCK
    }
}

impl std::fmt::Display for GmTicketType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::STUCK => f.write_str("STUCK"),
            Self::BEHAVIOR_HARASSMENT => f.write_str("BEHAVIOR_HARASSMENT"),
            Self::GUILD => f.write_str("GUILD"),
            Self::ITEM => f.write_str("ITEM"),
            Self::ENVIRONMENTAL => f.write_str("ENVIRONMENTAL"),
            Self::NONQUEST_CREEP => f.write_str("NONQUEST_CREEP"),
            Self::QUEST_QUESTNPC => f.write_str("QUEST_QUESTNPC"),
            Self::TECHNICAL => f.write_str("TECHNICAL"),
            Self::ACCOUNT_BILLING => f.write_str("ACCOUNT_BILLING"),
            Self::CHARACTER => f.write_str("CHARACTER"),
        }
    }
}

impl TryFrom<u8> for GmTicketType {
    type Error = TryFromGmTicketTypeError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::STUCK),
            2 => Ok(Self::BEHAVIOR_HARASSMENT),
            3 => Ok(Self::GUILD),
            4 => Ok(Self::ITEM),
            5 => Ok(Self::ENVIRONMENTAL),
            6 => Ok(Self::NONQUEST_CREEP),
            7 => Ok(Self::QUEST_QUESTNPC),
            8 => Ok(Self::TECHNICAL),
            9 => Ok(Self::ACCOUNT_BILLING),
            10 => Ok(Self::CHARACTER),
            _ => Err(TryFromGmTicketTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromGmTicketTypeError {
    value: u8,
}

impl TryFromGmTicketTypeError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum GmTicketTypeError {
    Read(std::io::Error),
    TryFrom(TryFromGmTicketTypeError),
}

impl std::error::Error for GmTicketTypeError {}
impl std::fmt::Display for TryFromGmTicketTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'GmTicketType': '{}'", self.value))
    }
}

impl std::fmt::Display for GmTicketTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for GmTicketTypeError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromGmTicketTypeError> for GmTicketTypeError {
    fn from(value: TryFromGmTicketTypeError) -> Self {
        Self::TryFrom(value)
    }
}

