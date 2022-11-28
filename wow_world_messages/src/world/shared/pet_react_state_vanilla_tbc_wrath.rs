use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/pet_common.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/pet_common.wowm#L1):
/// ```text
/// enum PetReactState : u8 {
///     PASSIVE = 0;
///     DEFENSIVE = 1;
///     AGGRESSIVE = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PetReactState {
    Passive,
    Defensive,
    Aggressive,
}

impl PetReactState {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Passive => 0x0,
            Self::Defensive => 0x1,
            Self::Aggressive => 0x2,
        }
    }

}

impl Default for PetReactState {
    fn default() -> Self {
        Self::Passive
    }
}

impl std::fmt::Display for PetReactState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Passive => f.write_str("Passive"),
            Self::Defensive => f.write_str("Defensive"),
            Self::Aggressive => f.write_str("Aggressive"),
        }
    }
}

impl TryFrom<u8> for PetReactState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Passive),
            1 => Ok(Self::Defensive),
            2 => Ok(Self::Aggressive),
            v => Err(crate::errors::EnumError::new("PetReactState", v as u32),)
        }
    }
}

