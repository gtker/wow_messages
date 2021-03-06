use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pvp/msg_pvp_log_data_server.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pvp/msg_pvp_log_data_server.wowm#L8):
/// ```text
/// enum BattlegroundWinner : u8 {
///     HORDE = 0;
///     ALLIANCE = 1;
///     NONE = 2;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum BattlegroundWinner {
    HORDE,
    ALLIANCE,
    NONE,
}

impl BattlegroundWinner {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::HORDE => 0x0,
            Self::ALLIANCE => 0x1,
            Self::NONE => 0x2,
        }
    }

}

impl Default for BattlegroundWinner {
    fn default() -> Self {
        Self::HORDE
    }
}

impl std::fmt::Display for BattlegroundWinner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HORDE => f.write_str("HORDE"),
            Self::ALLIANCE => f.write_str("ALLIANCE"),
            Self::NONE => f.write_str("NONE"),
        }
    }
}

impl TryFrom<u8> for BattlegroundWinner {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::HORDE),
            1 => Ok(Self::ALLIANCE),
            2 => Ok(Self::NONE),
            v => Err(crate::errors::EnumError::new("BattlegroundWinner", v as u32),)
        }
    }
}

