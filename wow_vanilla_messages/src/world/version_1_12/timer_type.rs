use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/spell_common.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/spell_common.wowm#L3):
/// ```text
/// enum TimerType : u32 {
///     FATIGUE = 0;
///     BREATH = 1;
///     FEIGNDEATH = 2;
///     ENVIRONMENTAL = 3;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum TimerType {
    FATIGUE,
    BREATH,
    FEIGNDEATH,
    /// Might be a mangos only thing.
    ///
    ENVIRONMENTAL,
}

impl TimerType {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::FATIGUE => 0x0,
            Self::BREATH => 0x1,
            Self::FEIGNDEATH => 0x2,
            Self::ENVIRONMENTAL => 0x3,
        }
    }

}

impl Default for TimerType {
    fn default() -> Self {
        Self::FATIGUE
    }
}

impl std::fmt::Display for TimerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FATIGUE => f.write_str("FATIGUE"),
            Self::BREATH => f.write_str("BREATH"),
            Self::FEIGNDEATH => f.write_str("FEIGNDEATH"),
            Self::ENVIRONMENTAL => f.write_str("ENVIRONMENTAL"),
        }
    }
}

impl TryFrom<u32> for TimerType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::FATIGUE),
            1 => Ok(Self::BREATH),
            2 => Ok(Self::FEIGNDEATH),
            3 => Ok(Self::ENVIRONMENTAL),
            v => Err(crate::errors::EnumError::new("TimerType", v as u32),)
        }
    }
}

