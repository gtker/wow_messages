/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_guild_roster.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_roster.wowm#L1):
/// ```text
/// enum GuildMemberStatus : u8 {
///     OFFLINE = 0;
///     ONLINE = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum GuildMemberStatus {
    Offline,
    Online,
}

impl GuildMemberStatus {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Offline => 0x0,
            Self::Online => 0x1,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl GuildMemberStatus {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Offline => "OFFLINE",
            Self::Online => "ONLINE",
        }
    }

}

impl Default for GuildMemberStatus {
    fn default() -> Self {
        Self::Offline
    }
}

impl std::fmt::Display for GuildMemberStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Offline => f.write_str("Offline"),
            Self::Online => f.write_str("Online"),
        }
    }
}

impl TryFrom<u8> for GuildMemberStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Offline),
            1 => Ok(Self::Online),
            v => Err(crate::errors::EnumError::new("GuildMemberStatus", v.into()),)
        }
    }
}

