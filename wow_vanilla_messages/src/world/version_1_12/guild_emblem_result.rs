use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/msg_save_guild_emblem_server.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/msg_save_guild_emblem_server.wowm#L3):
/// ```text
/// enum GuildEmblemResult : u32 {
///     SUCCESS = 0;
///     INVALID_TABARD_COLORS = 1;
///     NO_GUILD = 2;
///     NOT_GUILD_MASTER = 3;
///     NOT_ENOUGH_MONEY = 4;
///     NO_MESSAGE = 5;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GuildEmblemResult {
    /// Guild Emblem saved.
    ///
    SUCCESS,
    INVALID_TABARD_COLORS,
    /// vmangos: You are not part of a guild!
    ///
    NO_GUILD,
    /// vmangos: Only guild leaders can create emblems.
    ///
    NOT_GUILD_MASTER,
    /// vmangos: You can't afford to do that.
    ///
    NOT_ENOUGH_MONEY,
    /// mangoszero: This version fails silently.
    ///
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
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SUCCESS),
            1 => Ok(Self::INVALID_TABARD_COLORS),
            2 => Ok(Self::NO_GUILD),
            3 => Ok(Self::NOT_GUILD_MASTER),
            4 => Ok(Self::NOT_ENOUGH_MONEY),
            5 => Ok(Self::NO_MESSAGE),
            v => Err(crate::errors::EnumError::new("GuildEmblemResult", v as u32),)
        }
    }
}

