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
    Fatigue,
    Breath,
    Feigndeath,
    /// Might be a mangos only thing.
    ///
    Environmental,
}

impl TimerType {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Fatigue => 0x0,
            Self::Breath => 0x1,
            Self::Feigndeath => 0x2,
            Self::Environmental => 0x3,
        }
    }

}

impl Default for TimerType {
    fn default() -> Self {
        Self::Fatigue
    }
}

impl std::fmt::Display for TimerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fatigue => f.write_str("Fatigue"),
            Self::Breath => f.write_str("Breath"),
            Self::Feigndeath => f.write_str("Feigndeath"),
            Self::Environmental => f.write_str("Environmental"),
        }
    }
}

impl TryFrom<u32> for TimerType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Fatigue),
            1 => Ok(Self::Breath),
            2 => Ok(Self::Feigndeath),
            3 => Ok(Self::Environmental),
            v => Err(crate::errors::EnumError::new("TimerType", v as u32),)
        }
    }
}

