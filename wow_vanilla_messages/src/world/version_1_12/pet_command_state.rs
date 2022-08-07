use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/pet_common.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/pet_common.wowm#L9):
/// ```text
/// enum PetCommandState : u8 {
///     STAY = 0;
///     FOLLOW = 1;
///     ATTACK = 2;
///     DISMISS = 3;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PetCommandState {
    Stay,
    Follow,
    Attack,
    Dismiss,
}

impl PetCommandState {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Stay => 0x0,
            Self::Follow => 0x1,
            Self::Attack => 0x2,
            Self::Dismiss => 0x3,
        }
    }

}

impl Default for PetCommandState {
    fn default() -> Self {
        Self::Stay
    }
}

impl std::fmt::Display for PetCommandState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stay => f.write_str("Stay"),
            Self::Follow => f.write_str("Follow"),
            Self::Attack => f.write_str("Attack"),
            Self::Dismiss => f.write_str("Dismiss"),
        }
    }
}

impl TryFrom<u8> for PetCommandState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Stay),
            1 => Ok(Self::Follow),
            2 => Ok(Self::Attack),
            3 => Ok(Self::Dismiss),
            v => Err(crate::errors::EnumError::new("PetCommandState", v as u32),)
        }
    }
}

