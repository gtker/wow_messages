use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

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

impl FriendResult {
    pub(crate) const fn as_int(&self) -> u8 {
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
    type Error = FriendResultError;
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
            _ => Err(FriendResultError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct FriendResultError {
    pub value: u8,
}

impl FriendResultError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for FriendResultError {}
impl std::fmt::Display for FriendResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'FriendResult': '{}'", self.value))
    }
}

