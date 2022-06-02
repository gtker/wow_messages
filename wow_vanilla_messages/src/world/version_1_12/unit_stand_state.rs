use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/social_common.wowm:295`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/social_common.wowm#L295):
/// ```text
/// enum UnitStandState : u8 {
///     STAND = 0;
///     SIT = 1;
///     SIT_CHAIR = 2;
///     SLEEP = 3;
///     SIT_LOW_CHAIR = 4;
///     SIT_MEDIUM_CHAIR = 5;
///     SIT_HIGH_CHAIR = 6;
///     DEAD = 7;
///     KNEEL = 8;
///     CUSTOM = 9;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum UnitStandState {
    STAND,
    SIT,
    SIT_CHAIR,
    SLEEP,
    SIT_LOW_CHAIR,
    SIT_MEDIUM_CHAIR,
    SIT_HIGH_CHAIR,
    DEAD,
    KNEEL,
    /// Used for Cthun according to cmangos.
    ///
    CUSTOM,
}

impl UnitStandState {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::STAND => 0x0,
            Self::SIT => 0x1,
            Self::SIT_CHAIR => 0x2,
            Self::SLEEP => 0x3,
            Self::SIT_LOW_CHAIR => 0x4,
            Self::SIT_MEDIUM_CHAIR => 0x5,
            Self::SIT_HIGH_CHAIR => 0x6,
            Self::DEAD => 0x7,
            Self::KNEEL => 0x8,
            Self::CUSTOM => 0x9,
        }
    }

}

impl Default for UnitStandState {
    fn default() -> Self {
        Self::STAND
    }
}

impl std::fmt::Display for UnitStandState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::STAND => f.write_str("STAND"),
            Self::SIT => f.write_str("SIT"),
            Self::SIT_CHAIR => f.write_str("SIT_CHAIR"),
            Self::SLEEP => f.write_str("SLEEP"),
            Self::SIT_LOW_CHAIR => f.write_str("SIT_LOW_CHAIR"),
            Self::SIT_MEDIUM_CHAIR => f.write_str("SIT_MEDIUM_CHAIR"),
            Self::SIT_HIGH_CHAIR => f.write_str("SIT_HIGH_CHAIR"),
            Self::DEAD => f.write_str("DEAD"),
            Self::KNEEL => f.write_str("KNEEL"),
            Self::CUSTOM => f.write_str("CUSTOM"),
        }
    }
}

impl TryFrom<u8> for UnitStandState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::STAND),
            1 => Ok(Self::SIT),
            2 => Ok(Self::SIT_CHAIR),
            3 => Ok(Self::SLEEP),
            4 => Ok(Self::SIT_LOW_CHAIR),
            5 => Ok(Self::SIT_MEDIUM_CHAIR),
            6 => Ok(Self::SIT_HIGH_CHAIR),
            7 => Ok(Self::DEAD),
            8 => Ok(Self::KNEEL),
            9 => Ok(Self::CUSTOM),
            v => Err(crate::errors::EnumError::new("UnitStandState", v as u32),)
        }
    }
}

