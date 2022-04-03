use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/add_messages.wowm:218`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/add_messages.wowm#L218):
/// ```text
/// enum FriendResult : u8 {
///     DB_ERROR = 0x00;
///     LIST_FULL = 0x01;
///     ONLINE = 0x02;
///     OFFLINE = 0x03;
///     NOT_FOUND = 0x04;
///     REMOVED = 0x05;
///     ADDED_ONLINE = 0x06;
///     ADDED_OFFLINE = 0x07;
///     ALREADY = 0x08;
///     SELF = 0x09;
///     ENEMY = 0x0A;
///     IGNORE_FULL = 0x0B;
///     IGNORE_SELF = 0x0C;
///     IGNORE_NOT_FOUND = 0x0D;
///     IGNORE_ALREADY = 0x0E;
///     IGNORE_ADDED = 0x0F;
///     IGNORE_REMOVED = 0x10;
///     IGNORE_AMBIGUOUS = 0x11;
///     MUTE_FULL = 0x12;
///     MUTE_SELF = 0x13;
///     MUTE_NOT_FOUND = 0x14;
///     MUTE_ALREADY = 0x15;
///     MUTE_ADDED = 0x16;
///     MUTE_REMOVED = 0x17;
///     MUTE_AMBIGUOUS = 0x18;
///     UNKNOWN19 = 0x19;
///     UNKNOWN20 = 0x1A;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum FriendResult {
    DB_ERROR,
    LIST_FULL,
    ONLINE,
    OFFLINE,
    NOT_FOUND,
    REMOVED,
    ADDED_ONLINE,
    ADDED_OFFLINE,
    ALREADY,
    SELF,
    ENEMY,
    IGNORE_FULL,
    IGNORE_SELF,
    IGNORE_NOT_FOUND,
    IGNORE_ALREADY,
    IGNORE_ADDED,
    IGNORE_REMOVED,
    IGNORE_AMBIGUOUS,
    MUTE_FULL,
    MUTE_SELF,
    MUTE_NOT_FOUND,
    MUTE_ALREADY,
    MUTE_ADDED,
    MUTE_REMOVED,
    MUTE_AMBIGUOUS,
    UNKNOWN19,
    UNKNOWN20,
}

impl ReadableAndWritable for FriendResult {
    type Error = FriendResultError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl FriendResult {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, FriendResultError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, FriendResultError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, FriendResultError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, FriendResultError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, FriendResultError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, FriendResultError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::DB_ERROR => 0x0,
            Self::LIST_FULL => 0x1,
            Self::ONLINE => 0x2,
            Self::OFFLINE => 0x3,
            Self::NOT_FOUND => 0x4,
            Self::REMOVED => 0x5,
            Self::ADDED_ONLINE => 0x6,
            Self::ADDED_OFFLINE => 0x7,
            Self::ALREADY => 0x8,
            Self::SELF => 0x9,
            Self::ENEMY => 0xa,
            Self::IGNORE_FULL => 0xb,
            Self::IGNORE_SELF => 0xc,
            Self::IGNORE_NOT_FOUND => 0xd,
            Self::IGNORE_ALREADY => 0xe,
            Self::IGNORE_ADDED => 0xf,
            Self::IGNORE_REMOVED => 0x10,
            Self::IGNORE_AMBIGUOUS => 0x11,
            Self::MUTE_FULL => 0x12,
            Self::MUTE_SELF => 0x13,
            Self::MUTE_NOT_FOUND => 0x14,
            Self::MUTE_ALREADY => 0x15,
            Self::MUTE_ADDED => 0x16,
            Self::MUTE_REMOVED => 0x17,
            Self::MUTE_AMBIGUOUS => 0x18,
            Self::UNKNOWN19 => 0x19,
            Self::UNKNOWN20 => 0x1a,
        }
    }

    pub const fn new() -> Self {
        Self::DB_ERROR
    }

}

impl ConstantSized for FriendResult {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for FriendResult {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for FriendResult {
    fn default() -> Self {
        Self::DB_ERROR
    }
}

impl std::fmt::Display for FriendResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DB_ERROR => f.write_str("DB_ERROR"),
            Self::LIST_FULL => f.write_str("LIST_FULL"),
            Self::ONLINE => f.write_str("ONLINE"),
            Self::OFFLINE => f.write_str("OFFLINE"),
            Self::NOT_FOUND => f.write_str("NOT_FOUND"),
            Self::REMOVED => f.write_str("REMOVED"),
            Self::ADDED_ONLINE => f.write_str("ADDED_ONLINE"),
            Self::ADDED_OFFLINE => f.write_str("ADDED_OFFLINE"),
            Self::ALREADY => f.write_str("ALREADY"),
            Self::SELF => f.write_str("SELF"),
            Self::ENEMY => f.write_str("ENEMY"),
            Self::IGNORE_FULL => f.write_str("IGNORE_FULL"),
            Self::IGNORE_SELF => f.write_str("IGNORE_SELF"),
            Self::IGNORE_NOT_FOUND => f.write_str("IGNORE_NOT_FOUND"),
            Self::IGNORE_ALREADY => f.write_str("IGNORE_ALREADY"),
            Self::IGNORE_ADDED => f.write_str("IGNORE_ADDED"),
            Self::IGNORE_REMOVED => f.write_str("IGNORE_REMOVED"),
            Self::IGNORE_AMBIGUOUS => f.write_str("IGNORE_AMBIGUOUS"),
            Self::MUTE_FULL => f.write_str("MUTE_FULL"),
            Self::MUTE_SELF => f.write_str("MUTE_SELF"),
            Self::MUTE_NOT_FOUND => f.write_str("MUTE_NOT_FOUND"),
            Self::MUTE_ALREADY => f.write_str("MUTE_ALREADY"),
            Self::MUTE_ADDED => f.write_str("MUTE_ADDED"),
            Self::MUTE_REMOVED => f.write_str("MUTE_REMOVED"),
            Self::MUTE_AMBIGUOUS => f.write_str("MUTE_AMBIGUOUS"),
            Self::UNKNOWN19 => f.write_str("UNKNOWN19"),
            Self::UNKNOWN20 => f.write_str("UNKNOWN20"),
        }
    }
}

impl TryFrom<u8> for FriendResult {
    type Error = TryFromFriendResultError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DB_ERROR),
            1 => Ok(Self::LIST_FULL),
            2 => Ok(Self::ONLINE),
            3 => Ok(Self::OFFLINE),
            4 => Ok(Self::NOT_FOUND),
            5 => Ok(Self::REMOVED),
            6 => Ok(Self::ADDED_ONLINE),
            7 => Ok(Self::ADDED_OFFLINE),
            8 => Ok(Self::ALREADY),
            9 => Ok(Self::SELF),
            10 => Ok(Self::ENEMY),
            11 => Ok(Self::IGNORE_FULL),
            12 => Ok(Self::IGNORE_SELF),
            13 => Ok(Self::IGNORE_NOT_FOUND),
            14 => Ok(Self::IGNORE_ALREADY),
            15 => Ok(Self::IGNORE_ADDED),
            16 => Ok(Self::IGNORE_REMOVED),
            17 => Ok(Self::IGNORE_AMBIGUOUS),
            18 => Ok(Self::MUTE_FULL),
            19 => Ok(Self::MUTE_SELF),
            20 => Ok(Self::MUTE_NOT_FOUND),
            21 => Ok(Self::MUTE_ALREADY),
            22 => Ok(Self::MUTE_ADDED),
            23 => Ok(Self::MUTE_REMOVED),
            24 => Ok(Self::MUTE_AMBIGUOUS),
            25 => Ok(Self::UNKNOWN19),
            26 => Ok(Self::UNKNOWN20),
            _ => Err(TryFromFriendResultError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromFriendResultError {
    value: u8,
}

impl TryFromFriendResultError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum FriendResultError {
    Read(std::io::Error),
    TryFrom(TryFromFriendResultError),
}

impl std::error::Error for FriendResultError {}
impl std::fmt::Display for TryFromFriendResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'FriendResult': '{}'", self.value))
    }
}

impl std::fmt::Display for FriendResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for FriendResultError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromFriendResultError> for FriendResultError {
    fn from(value: TryFromFriendResultError) -> Self {
        Self::TryFrom(value)
    }
}

