use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum FriendStatus {
    OFFLINE,
    ONLINE,
    AFK,
    UNKNOWN3,
    DND,
}

impl FriendStatus {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::OFFLINE => 0x0,
            Self::ONLINE => 0x1,
            Self::AFK => 0x2,
            Self::UNKNOWN3 => 0x3,
            Self::DND => 0x4,
        }
    }

}

impl Default for FriendStatus {
    fn default() -> Self {
        Self::OFFLINE
    }
}

impl std::fmt::Display for FriendStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OFFLINE => f.write_str("OFFLINE"),
            Self::ONLINE => f.write_str("ONLINE"),
            Self::AFK => f.write_str("AFK"),
            Self::UNKNOWN3 => f.write_str("UNKNOWN3"),
            Self::DND => f.write_str("DND"),
        }
    }
}

impl TryFrom<u8> for FriendStatus {
    type Error = FriendStatusError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OFFLINE),
            1 => Ok(Self::ONLINE),
            2 => Ok(Self::AFK),
            3 => Ok(Self::UNKNOWN3),
            4 => Ok(Self::DND),
            _ => Err(FriendStatusError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct FriendStatusError {
    value: u8,
}

impl FriendStatusError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for FriendStatusError {}
impl std::fmt::Display for FriendStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'FriendStatus': '{}'", self.value))
    }
}

