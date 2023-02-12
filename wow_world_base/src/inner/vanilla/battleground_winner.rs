/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pvp/msg_pvp_log_data_server.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pvp/msg_pvp_log_data_server.wowm#L8):
/// ```text
/// enum BattlegroundWinner : u8 {
///     HORDE = 0;
///     ALLIANCE = 1;
///     NONE = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum BattlegroundWinner {
    Horde,
    Alliance,
    None,
}

impl BattlegroundWinner {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Horde => 0x0,
            Self::Alliance => 0x1,
            Self::None => 0x2,
        }
    }

}

impl Default for BattlegroundWinner {
    fn default() -> Self {
        Self::Horde
    }
}

impl std::fmt::Display for BattlegroundWinner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Horde => f.write_str("Horde"),
            Self::Alliance => f.write_str("Alliance"),
            Self::None => f.write_str("None"),
        }
    }
}

impl TryFrom<u8> for BattlegroundWinner {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Horde),
            1 => Ok(Self::Alliance),
            2 => Ok(Self::None),
            v => Err(crate::errors::EnumError::new("BattlegroundWinner", v as u64),)
        }
    }
}

