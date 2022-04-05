use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/needs_msg.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/needs_msg.wowm#L9):
/// ```text
/// enum QuestPartyMessage : u8 {
///     SHARING_QUEST = 0;
///     CANT_TAKE_QUEST = 1;
///     ACCEPT_QUEST = 2;
///     DECLINE_QUEST = 3;
///     TOO_FAR = 4;
///     BUSY = 5;
///     LOG_FULL = 6;
///     HAVE_QUEST = 7;
///     FINISH_QUEST = 8;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum QuestPartyMessage {
    SHARING_QUEST,
    CANT_TAKE_QUEST,
    ACCEPT_QUEST,
    DECLINE_QUEST,
    TOO_FAR,
    BUSY,
    LOG_FULL,
    HAVE_QUEST,
    FINISH_QUEST,
}

impl ReadableAndWritable for QuestPartyMessage {
    type Error = QuestPartyMessageError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl QuestPartyMessage {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, QuestPartyMessageError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, QuestPartyMessageError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, QuestPartyMessageError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, QuestPartyMessageError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, QuestPartyMessageError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, QuestPartyMessageError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::SHARING_QUEST => 0x0,
            Self::CANT_TAKE_QUEST => 0x1,
            Self::ACCEPT_QUEST => 0x2,
            Self::DECLINE_QUEST => 0x3,
            Self::TOO_FAR => 0x4,
            Self::BUSY => 0x5,
            Self::LOG_FULL => 0x6,
            Self::HAVE_QUEST => 0x7,
            Self::FINISH_QUEST => 0x8,
        }
    }

    pub const fn new() -> Self {
        Self::SHARING_QUEST
    }

}

impl ConstantSized for QuestPartyMessage {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for QuestPartyMessage {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for QuestPartyMessage {
    fn default() -> Self {
        Self::SHARING_QUEST
    }
}

impl std::fmt::Display for QuestPartyMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SHARING_QUEST => f.write_str("SHARING_QUEST"),
            Self::CANT_TAKE_QUEST => f.write_str("CANT_TAKE_QUEST"),
            Self::ACCEPT_QUEST => f.write_str("ACCEPT_QUEST"),
            Self::DECLINE_QUEST => f.write_str("DECLINE_QUEST"),
            Self::TOO_FAR => f.write_str("TOO_FAR"),
            Self::BUSY => f.write_str("BUSY"),
            Self::LOG_FULL => f.write_str("LOG_FULL"),
            Self::HAVE_QUEST => f.write_str("HAVE_QUEST"),
            Self::FINISH_QUEST => f.write_str("FINISH_QUEST"),
        }
    }
}

impl TryFrom<u8> for QuestPartyMessage {
    type Error = TryFromQuestPartyMessageError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SHARING_QUEST),
            1 => Ok(Self::CANT_TAKE_QUEST),
            2 => Ok(Self::ACCEPT_QUEST),
            3 => Ok(Self::DECLINE_QUEST),
            4 => Ok(Self::TOO_FAR),
            5 => Ok(Self::BUSY),
            6 => Ok(Self::LOG_FULL),
            7 => Ok(Self::HAVE_QUEST),
            8 => Ok(Self::FINISH_QUEST),
            _ => Err(TryFromQuestPartyMessageError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromQuestPartyMessageError {
    value: u8,
}

impl TryFromQuestPartyMessageError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum QuestPartyMessageError {
    Read(std::io::Error),
    TryFrom(TryFromQuestPartyMessageError),
}

impl std::error::Error for QuestPartyMessageError {}
impl std::fmt::Display for TryFromQuestPartyMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'QuestPartyMessage': '{}'", self.value))
    }
}

impl std::fmt::Display for QuestPartyMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for QuestPartyMessageError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromQuestPartyMessageError> for QuestPartyMessageError {
    fn from(value: TryFromQuestPartyMessageError) -> Self {
        Self::TryFrom(value)
    }
}

