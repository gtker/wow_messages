use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GuildEmblemResult {
    SUCCESS,
    INVALID_TABARD_COLORS,
    NO_GUILD,
    NOT_GUILD_MASTER,
    NOT_ENOUGH_MONEY,
    NO_MESSAGE,
}

impl GuildEmblemResult {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::SUCCESS => 0x0,
            Self::INVALID_TABARD_COLORS => 0x1,
            Self::NO_GUILD => 0x2,
            Self::NOT_GUILD_MASTER => 0x3,
            Self::NOT_ENOUGH_MONEY => 0x4,
            Self::NO_MESSAGE => 0x5,
        }
    }

}

impl Default for GuildEmblemResult {
    fn default() -> Self {
        Self::SUCCESS
    }
}

impl std::fmt::Display for GuildEmblemResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SUCCESS => f.write_str("SUCCESS"),
            Self::INVALID_TABARD_COLORS => f.write_str("INVALID_TABARD_COLORS"),
            Self::NO_GUILD => f.write_str("NO_GUILD"),
            Self::NOT_GUILD_MASTER => f.write_str("NOT_GUILD_MASTER"),
            Self::NOT_ENOUGH_MONEY => f.write_str("NOT_ENOUGH_MONEY"),
            Self::NO_MESSAGE => f.write_str("NO_MESSAGE"),
        }
    }
}

impl TryFrom<u32> for GuildEmblemResult {
    type Error = GuildEmblemResultError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SUCCESS),
            1 => Ok(Self::INVALID_TABARD_COLORS),
            2 => Ok(Self::NO_GUILD),
            3 => Ok(Self::NOT_GUILD_MASTER),
            4 => Ok(Self::NOT_ENOUGH_MONEY),
            5 => Ok(Self::NO_MESSAGE),
            _ => Err(GuildEmblemResultError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct GuildEmblemResultError {
    pub value: u32,
}

impl GuildEmblemResultError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::error::Error for GuildEmblemResultError {}
impl std::fmt::Display for GuildEmblemResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'GuildEmblemResult': '{}'", self.value))
    }
}

