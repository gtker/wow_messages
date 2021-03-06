use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pvp/msg_pvp_log_data_server.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pvp/msg_pvp_log_data_server.wowm#L3):
/// ```text
/// enum BattlegroundEndStatus : u8 {
///     NOT_ENDED = 0;
///     ENDED = 1;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum BattlegroundEndStatus {
    NOT_ENDED,
    ENDED,
}

impl BattlegroundEndStatus {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NOT_ENDED => 0x0,
            Self::ENDED => 0x1,
        }
    }

}

impl Default for BattlegroundEndStatus {
    fn default() -> Self {
        Self::NOT_ENDED
    }
}

impl std::fmt::Display for BattlegroundEndStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NOT_ENDED => f.write_str("NOT_ENDED"),
            Self::ENDED => f.write_str("ENDED"),
        }
    }
}

impl TryFrom<u8> for BattlegroundEndStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NOT_ENDED),
            1 => Ok(Self::ENDED),
            v => Err(crate::errors::EnumError::new("BattlegroundEndStatus", v as u32),)
        }
    }
}

