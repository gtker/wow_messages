use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_trainer_list.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_trainer_list.wowm#L3):
/// ```text
/// enum TrainerSpellState : u8 {
///     GREEN = 0;
///     RED = 1;
///     GRAY = 2;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum TrainerSpellState {
    GREEN,
    RED,
    GRAY,
}

impl TrainerSpellState {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::GREEN => 0x0,
            Self::RED => 0x1,
            Self::GRAY => 0x2,
        }
    }

}

impl Default for TrainerSpellState {
    fn default() -> Self {
        Self::GREEN
    }
}

impl std::fmt::Display for TrainerSpellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GREEN => f.write_str("GREEN"),
            Self::RED => f.write_str("RED"),
            Self::GRAY => f.write_str("GRAY"),
        }
    }
}

impl TryFrom<u8> for TrainerSpellState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::GREEN),
            1 => Ok(Self::RED),
            2 => Ok(Self::GRAY),
            v => Err(crate::errors::EnumError::new("TrainerSpellState", v as u32),)
        }
    }
}

