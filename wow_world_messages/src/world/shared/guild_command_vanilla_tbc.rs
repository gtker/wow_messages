/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_guild_command_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_command_result.wowm#L1):
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
    Create,
    Invite,
    Quit,
    Founder,
    /// cmangos claims this triggers UI event EVENT_GUILD_ROSTER_UPDATE
    ///
    Unknown19,
    /// cmangos claims this triggers UI event EVENT_GUILD_ROSTER_UPDATE
    ///
    Unknown20,
}

impl GuildCommand {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Create => 0x0,
            Self::Invite => 0x1,
            Self::Quit => 0x3,
            Self::Founder => 0xe,
            Self::Unknown19 => 0x13,
            Self::Unknown20 => 0x14,
        }
    }

}

impl Default for GuildCommand {
    fn default() -> Self {
        Self::Create
    }
}

impl std::fmt::Display for GuildCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Create => f.write_str("Create"),
            Self::Invite => f.write_str("Invite"),
            Self::Quit => f.write_str("Quit"),
            Self::Founder => f.write_str("Founder"),
            Self::Unknown19 => f.write_str("Unknown19"),
            Self::Unknown20 => f.write_str("Unknown20"),
        }
    }
}

impl TryFrom<u8> for GuildCommand {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Create),
            1 => Ok(Self::Invite),
            3 => Ok(Self::Quit),
            14 => Ok(Self::Founder),
            19 => Ok(Self::Unknown19),
            20 => Ok(Self::Unknown20),
            v => Err(crate::errors::EnumError::new("GuildCommand", v as u64),)
        }
    }
}

