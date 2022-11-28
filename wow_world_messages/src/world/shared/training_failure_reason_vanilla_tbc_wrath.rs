use std::convert::{TryFrom, TryInto};

/// These errors are only printed in client console.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_trainer_buy_failed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_trainer_buy_failed.wowm#L3):
/// ```text
/// enum TrainingFailureReason : u32 {
///     UNAVAILABLE = 0;
///     NOT_ENOUGH_MONEY = 1;
///     NOT_ENOUGH_SKILL = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum TrainingFailureReason {
    /// Trainer service %d unavailable.
    ///
    Unavailable,
    /// Not enough money for trainer service %d.
    ///
    NotEnoughMoney,
    /// Not enough skill points for trainer service %d.
    ///
    NotEnoughSkill,
}

impl TrainingFailureReason {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Unavailable => 0x0,
            Self::NotEnoughMoney => 0x1,
            Self::NotEnoughSkill => 0x2,
        }
    }

}

impl Default for TrainingFailureReason {
    fn default() -> Self {
        Self::Unavailable
    }
}

impl std::fmt::Display for TrainingFailureReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unavailable => f.write_str("Unavailable"),
            Self::NotEnoughMoney => f.write_str("NotEnoughMoney"),
            Self::NotEnoughSkill => f.write_str("NotEnoughSkill"),
        }
    }
}

impl TryFrom<u32> for TrainingFailureReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unavailable),
            1 => Ok(Self::NotEnoughMoney),
            2 => Ok(Self::NotEnoughSkill),
            v => Err(crate::errors::EnumError::new("TrainingFailureReason", v as u32),)
        }
    }
}

