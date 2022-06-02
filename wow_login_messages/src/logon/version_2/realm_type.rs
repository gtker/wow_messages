use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_realm/server.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_realm/server.wowm#L2):
/// ```text
/// enum RealmType : u8 {
///     PLAYER_VS_ENVIRONMENT = 0;
///     PLAYER_VS_PLAYER = 1;
///     ROLEPLAYING = 6;
///     ROLEPLAYING_PLAYER_VS_PLAYER = 8;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum RealmType {
    PLAYER_VS_ENVIRONMENT,
    PLAYER_VS_PLAYER,
    ROLEPLAYING,
    ROLEPLAYING_PLAYER_VS_PLAYER,
}

impl RealmType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::PLAYER_VS_ENVIRONMENT => 0x0,
            Self::PLAYER_VS_PLAYER => 0x1,
            Self::ROLEPLAYING => 0x6,
            Self::ROLEPLAYING_PLAYER_VS_PLAYER => 0x8,
        }
    }

}

impl Default for RealmType {
    fn default() -> Self {
        Self::PLAYER_VS_ENVIRONMENT
    }
}

impl std::fmt::Display for RealmType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PLAYER_VS_ENVIRONMENT => f.write_str("PLAYER_VS_ENVIRONMENT"),
            Self::PLAYER_VS_PLAYER => f.write_str("PLAYER_VS_PLAYER"),
            Self::ROLEPLAYING => f.write_str("ROLEPLAYING"),
            Self::ROLEPLAYING_PLAYER_VS_PLAYER => f.write_str("ROLEPLAYING_PLAYER_VS_PLAYER"),
        }
    }
}

impl TryFrom<u8> for RealmType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PLAYER_VS_ENVIRONMENT),
            1 => Ok(Self::PLAYER_VS_PLAYER),
            6 => Ok(Self::ROLEPLAYING),
            8 => Ok(Self::ROLEPLAYING_PLAYER_VS_PLAYER),
            v => Err(crate::errors::EnumError::new("RealmType", v as u32),)
        }
    }
}

