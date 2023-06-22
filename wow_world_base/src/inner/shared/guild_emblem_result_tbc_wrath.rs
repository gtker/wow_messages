/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/msg_save_guild_emblem_server.wowm:22`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/msg_save_guild_emblem_server.wowm#L22):
/// ```text
/// enum GuildEmblemResult : u32 {
///     SUCCESS = 0;
///     INVALID_TABARD_COLORS = 1;
///     NO_GUILD = 2;
///     NOT_GUILD_MASTER = 3;
///     NOT_ENOUGH_MONEY = 4;
///     INVALID_VENDOR = 5;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum GuildEmblemResult {
    Success,
    InvalidTabardColors,
    NoGuild,
    NotGuildMaster,
    NotEnoughMoney,
    InvalidVendor,
}

impl GuildEmblemResult {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Success => 0x0,
            Self::InvalidTabardColors => 0x1,
            Self::NoGuild => 0x2,
            Self::NotGuildMaster => 0x3,
            Self::NotEnoughMoney => 0x4,
            Self::InvalidVendor => 0x5,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl GuildEmblemResult {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Success => "SUCCESS",
            Self::InvalidTabardColors => "INVALID_TABARD_COLORS",
            Self::NoGuild => "NO_GUILD",
            Self::NotGuildMaster => "NOT_GUILD_MASTER",
            Self::NotEnoughMoney => "NOT_ENOUGH_MONEY",
            Self::InvalidVendor => "INVALID_VENDOR",
        }
    }

}

const NAME: &str = "GuildEmblemResult";

impl Default for GuildEmblemResult {
    fn default() -> Self {
        Self::Success
    }
}

impl std::fmt::Display for GuildEmblemResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => f.write_str("Success"),
            Self::InvalidTabardColors => f.write_str("InvalidTabardColors"),
            Self::NoGuild => f.write_str("NoGuild"),
            Self::NotGuildMaster => f.write_str("NotGuildMaster"),
            Self::NotEnoughMoney => f.write_str("NotEnoughMoney"),
            Self::InvalidVendor => f.write_str("InvalidVendor"),
        }
    }
}

impl TryFrom<u32> for GuildEmblemResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Success),
            1 => Ok(Self::InvalidTabardColors),
            2 => Ok(Self::NoGuild),
            3 => Ok(Self::NotGuildMaster),
            4 => Ok(Self::NotEnoughMoney),
            5 => Ok(Self::InvalidVendor),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u8> for GuildEmblemResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u16> for GuildEmblemResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for GuildEmblemResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for GuildEmblemResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for GuildEmblemResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for GuildEmblemResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for GuildEmblemResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for GuildEmblemResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

