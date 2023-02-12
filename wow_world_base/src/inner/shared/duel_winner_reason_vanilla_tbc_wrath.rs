/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/duel/smsg_duel_winner.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/duel/smsg_duel_winner.wowm#L3):
/// ```text
/// enum DuelWinnerReason : u8 {
///     WON = 0;
///     FLED = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum DuelWinnerReason {
    Won,
    Fled,
}

impl DuelWinnerReason {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Won => 0x0,
            Self::Fled => 0x1,
        }
    }

}

impl Default for DuelWinnerReason {
    fn default() -> Self {
        Self::Won
    }
}

impl std::fmt::Display for DuelWinnerReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Won => f.write_str("Won"),
            Self::Fled => f.write_str("Fled"),
        }
    }
}

impl TryFrom<u8> for DuelWinnerReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Won),
            1 => Ok(Self::Fled),
            v => Err(crate::errors::EnumError::new("DuelWinnerReason", v as u64),)
        }
    }
}

