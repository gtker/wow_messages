/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_guild_command_result.wowm:29`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_command_result.wowm#L29):
/// ```text
/// enum GuildCommandResult : u8 {
///     PLAYER_NO_MORE_IN_GUILD = 0x00;
///     GUILD_INTERNAL = 0x01;
///     ALREADY_IN_GUILD = 0x02;
///     ALREADY_IN_GUILD_S = 0x03;
///     INVITED_TO_GUILD = 0x04;
///     ALREADY_INVITED_TO_GUILD_S = 0x05;
///     GUILD_NAME_INVALID = 0x06;
///     GUILD_NAME_EXISTS_S = 0x07;
///     GUILD_PERMISSIONS_OR_LEADER = 0x08;
///     GUILD_PLAYER_NOT_IN_GUILD = 0x09;
///     GUILD_PLAYER_NOT_IN_GUILD_S = 0x0A;
///     GUILD_PLAYER_NOT_FOUND_S = 0x0B;
///     GUILD_NOT_ALLIED = 0x0C;
///     GUILD_RANK_TOO_HIGH_S = 0x0D;
///     GUILD_RANK_TOO_LOW_S = 0x0E;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum GuildCommandResult {
    PlayerNoMoreInGuild,
    GuildInternal,
    AlreadyInGuild,
    AlreadyInGuildS,
    InvitedToGuild,
    AlreadyInvitedToGuildS,
    GuildNameInvalid,
    GuildNameExistsS,
    /// mangos has 0x08 as both GUILD_LEADER_LEAVE and GUILD_PERMISSIONS.
    /// Supposedly the [`GuildCommand`](crate::vanilla::GuildCommand) QUIT used GUILD_LEADER_LEAVE and others used GUILD_PERMISSIONS
    GuildPermissionsOrLeader,
    GuildPlayerNotInGuild,
    GuildPlayerNotInGuildS,
    GuildPlayerNotFoundS,
    GuildNotAllied,
    GuildRankTooHighS,
    GuildRankTooLowS,
}

impl GuildCommandResult {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::PlayerNoMoreInGuild => 0x0,
            Self::GuildInternal => 0x1,
            Self::AlreadyInGuild => 0x2,
            Self::AlreadyInGuildS => 0x3,
            Self::InvitedToGuild => 0x4,
            Self::AlreadyInvitedToGuildS => 0x5,
            Self::GuildNameInvalid => 0x6,
            Self::GuildNameExistsS => 0x7,
            Self::GuildPermissionsOrLeader => 0x8,
            Self::GuildPlayerNotInGuild => 0x9,
            Self::GuildPlayerNotInGuildS => 0xa,
            Self::GuildPlayerNotFoundS => 0xb,
            Self::GuildNotAllied => 0xc,
            Self::GuildRankTooHighS => 0xd,
            Self::GuildRankTooLowS => 0xe,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl GuildCommandResult {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::PlayerNoMoreInGuild => "PLAYER_NO_MORE_IN_GUILD",
            Self::GuildInternal => "GUILD_INTERNAL",
            Self::AlreadyInGuild => "ALREADY_IN_GUILD",
            Self::AlreadyInGuildS => "ALREADY_IN_GUILD_S",
            Self::InvitedToGuild => "INVITED_TO_GUILD",
            Self::AlreadyInvitedToGuildS => "ALREADY_INVITED_TO_GUILD_S",
            Self::GuildNameInvalid => "GUILD_NAME_INVALID",
            Self::GuildNameExistsS => "GUILD_NAME_EXISTS_S",
            Self::GuildPermissionsOrLeader => "GUILD_PERMISSIONS_OR_LEADER",
            Self::GuildPlayerNotInGuild => "GUILD_PLAYER_NOT_IN_GUILD",
            Self::GuildPlayerNotInGuildS => "GUILD_PLAYER_NOT_IN_GUILD_S",
            Self::GuildPlayerNotFoundS => "GUILD_PLAYER_NOT_FOUND_S",
            Self::GuildNotAllied => "GUILD_NOT_ALLIED",
            Self::GuildRankTooHighS => "GUILD_RANK_TOO_HIGH_S",
            Self::GuildRankTooLowS => "GUILD_RANK_TOO_LOW_S",
        }
    }

}

const NAME: &str = "GuildCommandResult";

impl Default for GuildCommandResult {
    fn default() -> Self {
        Self::PlayerNoMoreInGuild
    }
}

impl std::fmt::Display for GuildCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PlayerNoMoreInGuild => f.write_str("PlayerNoMoreInGuild"),
            Self::GuildInternal => f.write_str("GuildInternal"),
            Self::AlreadyInGuild => f.write_str("AlreadyInGuild"),
            Self::AlreadyInGuildS => f.write_str("AlreadyInGuildS"),
            Self::InvitedToGuild => f.write_str("InvitedToGuild"),
            Self::AlreadyInvitedToGuildS => f.write_str("AlreadyInvitedToGuildS"),
            Self::GuildNameInvalid => f.write_str("GuildNameInvalid"),
            Self::GuildNameExistsS => f.write_str("GuildNameExistsS"),
            Self::GuildPermissionsOrLeader => f.write_str("GuildPermissionsOrLeader"),
            Self::GuildPlayerNotInGuild => f.write_str("GuildPlayerNotInGuild"),
            Self::GuildPlayerNotInGuildS => f.write_str("GuildPlayerNotInGuildS"),
            Self::GuildPlayerNotFoundS => f.write_str("GuildPlayerNotFoundS"),
            Self::GuildNotAllied => f.write_str("GuildNotAllied"),
            Self::GuildRankTooHighS => f.write_str("GuildRankTooHighS"),
            Self::GuildRankTooLowS => f.write_str("GuildRankTooLowS"),
        }
    }
}

impl TryFrom<u8> for GuildCommandResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PlayerNoMoreInGuild),
            1 => Ok(Self::GuildInternal),
            2 => Ok(Self::AlreadyInGuild),
            3 => Ok(Self::AlreadyInGuildS),
            4 => Ok(Self::InvitedToGuild),
            5 => Ok(Self::AlreadyInvitedToGuildS),
            6 => Ok(Self::GuildNameInvalid),
            7 => Ok(Self::GuildNameExistsS),
            8 => Ok(Self::GuildPermissionsOrLeader),
            9 => Ok(Self::GuildPlayerNotInGuild),
            10 => Ok(Self::GuildPlayerNotInGuildS),
            11 => Ok(Self::GuildPlayerNotFoundS),
            12 => Ok(Self::GuildNotAllied),
            13 => Ok(Self::GuildRankTooHighS),
            14 => Ok(Self::GuildRankTooLowS),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for GuildCommandResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for GuildCommandResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for GuildCommandResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for GuildCommandResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for GuildCommandResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for GuildCommandResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for GuildCommandResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for GuildCommandResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

