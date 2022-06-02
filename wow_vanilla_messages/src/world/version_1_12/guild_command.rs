use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_guild_command_result.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_command_result.wowm#L3):
/// ```text
/// enum GuildCommand : u8 {
///     CREATE = 0x00;
///     INVITE = 0x01;
///     QUIT = 0x03;
///     FOUNDER = 0x0E;
///     UNKNOWN19 = 0x13;
///     UNKNOWN20 = 0x14;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GuildCommand {
    CREATE,
    INVITE,
    QUIT,
    FOUNDER,
    /// # Comment
    ///
    /// cmangos claims this triggers UI event EVENT_GUILD_ROSTER_UPDATE
    UNKNOWN19,
    /// # Comment
    ///
    /// cmangos claims this triggers UI event EVENT_GUILD_ROSTER_UPDATE
    UNKNOWN20,
}

impl GuildCommand {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::CREATE => 0x0,
            Self::INVITE => 0x1,
            Self::QUIT => 0x3,
            Self::FOUNDER => 0xe,
            Self::UNKNOWN19 => 0x13,
            Self::UNKNOWN20 => 0x14,
        }
    }

}

impl Default for GuildCommand {
    fn default() -> Self {
        Self::CREATE
    }
}

impl std::fmt::Display for GuildCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CREATE => f.write_str("CREATE"),
            Self::INVITE => f.write_str("INVITE"),
            Self::QUIT => f.write_str("QUIT"),
            Self::FOUNDER => f.write_str("FOUNDER"),
            Self::UNKNOWN19 => f.write_str("UNKNOWN19"),
            Self::UNKNOWN20 => f.write_str("UNKNOWN20"),
        }
    }
}

impl TryFrom<u8> for GuildCommand {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::CREATE),
            1 => Ok(Self::INVITE),
            3 => Ok(Self::QUIT),
            14 => Ok(Self::FOUNDER),
            19 => Ok(Self::UNKNOWN19),
            20 => Ok(Self::UNKNOWN20),
            v => Err(crate::errors::EnumError::new("GuildCommand", v as u32),)
        }
    }
}

