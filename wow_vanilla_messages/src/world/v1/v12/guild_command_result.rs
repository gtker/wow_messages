use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GuildCommandResult {
    PLAYER_NO_MORE_IN_GUILD,
    GUILD_INTERNAL,
    ALREADY_IN_GUILD,
    ALREADY_IN_GUILD_S,
    INVITED_TO_GUILD,
    ALREADY_INVITED_TO_GUILD_S,
    GUILD_NAME_INVALID,
    GUILD_NAME_EXISTS_S,
    GUILD_LEADER_LEAVE,
    GUILD_PERMISSIONS,
    GUILD_PLAYER_NOT_IN_GUILD,
    GUILD_PLAYER_NOT_IN_GUILD_S,
    GUILD_PLAYER_NOT_FOUND_S,
    GUILD_NOT_ALLIED,
    GUILD_RANK_TOO_HIGH_S,
    GUILD_RANK_TOO_LOW_S,
}

impl GuildCommandResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::PLAYER_NO_MORE_IN_GUILD => 0x0,
            Self::GUILD_INTERNAL => 0x1,
            Self::ALREADY_IN_GUILD => 0x2,
            Self::ALREADY_IN_GUILD_S => 0x3,
            Self::INVITED_TO_GUILD => 0x4,
            Self::ALREADY_INVITED_TO_GUILD_S => 0x5,
            Self::GUILD_NAME_INVALID => 0x6,
            Self::GUILD_NAME_EXISTS_S => 0x7,
            Self::GUILD_LEADER_LEAVE => 0x8,
            Self::GUILD_PERMISSIONS => 0x8,
            Self::GUILD_PLAYER_NOT_IN_GUILD => 0x9,
            Self::GUILD_PLAYER_NOT_IN_GUILD_S => 0xa,
            Self::GUILD_PLAYER_NOT_FOUND_S => 0xb,
            Self::GUILD_NOT_ALLIED => 0xc,
            Self::GUILD_RANK_TOO_HIGH_S => 0xd,
            Self::GUILD_RANK_TOO_LOW_S => 0xe,
        }
    }

}

impl Default for GuildCommandResult {
    fn default() -> Self {
        Self::PLAYER_NO_MORE_IN_GUILD
    }
}

impl std::fmt::Display for GuildCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PLAYER_NO_MORE_IN_GUILD => f.write_str("PLAYER_NO_MORE_IN_GUILD"),
            Self::GUILD_INTERNAL => f.write_str("GUILD_INTERNAL"),
            Self::ALREADY_IN_GUILD => f.write_str("ALREADY_IN_GUILD"),
            Self::ALREADY_IN_GUILD_S => f.write_str("ALREADY_IN_GUILD_S"),
            Self::INVITED_TO_GUILD => f.write_str("INVITED_TO_GUILD"),
            Self::ALREADY_INVITED_TO_GUILD_S => f.write_str("ALREADY_INVITED_TO_GUILD_S"),
            Self::GUILD_NAME_INVALID => f.write_str("GUILD_NAME_INVALID"),
            Self::GUILD_NAME_EXISTS_S => f.write_str("GUILD_NAME_EXISTS_S"),
            Self::GUILD_LEADER_LEAVE => f.write_str("GUILD_LEADER_LEAVE"),
            Self::GUILD_PERMISSIONS => f.write_str("GUILD_PERMISSIONS"),
            Self::GUILD_PLAYER_NOT_IN_GUILD => f.write_str("GUILD_PLAYER_NOT_IN_GUILD"),
            Self::GUILD_PLAYER_NOT_IN_GUILD_S => f.write_str("GUILD_PLAYER_NOT_IN_GUILD_S"),
            Self::GUILD_PLAYER_NOT_FOUND_S => f.write_str("GUILD_PLAYER_NOT_FOUND_S"),
            Self::GUILD_NOT_ALLIED => f.write_str("GUILD_NOT_ALLIED"),
            Self::GUILD_RANK_TOO_HIGH_S => f.write_str("GUILD_RANK_TOO_HIGH_S"),
            Self::GUILD_RANK_TOO_LOW_S => f.write_str("GUILD_RANK_TOO_LOW_S"),
        }
    }
}

impl TryFrom<u8> for GuildCommandResult {
    type Error = GuildCommandResultError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PLAYER_NO_MORE_IN_GUILD),
            1 => Ok(Self::GUILD_INTERNAL),
            2 => Ok(Self::ALREADY_IN_GUILD),
            3 => Ok(Self::ALREADY_IN_GUILD_S),
            4 => Ok(Self::INVITED_TO_GUILD),
            5 => Ok(Self::ALREADY_INVITED_TO_GUILD_S),
            6 => Ok(Self::GUILD_NAME_INVALID),
            7 => Ok(Self::GUILD_NAME_EXISTS_S),
            8 => Ok(Self::GUILD_LEADER_LEAVE),
            8 => Ok(Self::GUILD_PERMISSIONS),
            9 => Ok(Self::GUILD_PLAYER_NOT_IN_GUILD),
            10 => Ok(Self::GUILD_PLAYER_NOT_IN_GUILD_S),
            11 => Ok(Self::GUILD_PLAYER_NOT_FOUND_S),
            12 => Ok(Self::GUILD_NOT_ALLIED),
            13 => Ok(Self::GUILD_RANK_TOO_HIGH_S),
            14 => Ok(Self::GUILD_RANK_TOO_LOW_S),
            _ => Err(GuildCommandResultError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct GuildCommandResultError {
    pub value: u8,
}

impl GuildCommandResultError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for GuildCommandResultError {}
impl std::fmt::Display for GuildCommandResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'GuildCommandResult': '{}'", self.value))
    }
}

