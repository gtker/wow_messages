use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

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
    STAY,
    FOLLOW,
    ATTACK,
    DISMISS,
}

impl PetCommandState {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::STAY => 0x0,
            Self::FOLLOW => 0x1,
            Self::ATTACK => 0x2,
            Self::DISMISS => 0x3,
        }
    }

}

impl Default for PetCommandState {
    fn default() -> Self {
        Self::STAY
    }
}

impl std::fmt::Display for PetCommandState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::STAY => f.write_str("STAY"),
            Self::FOLLOW => f.write_str("FOLLOW"),
            Self::ATTACK => f.write_str("ATTACK"),
            Self::DISMISS => f.write_str("DISMISS"),
        }
    }
}

impl TryFrom<u8> for PetCommandState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::STAY),
            1 => Ok(Self::FOLLOW),
            2 => Ok(Self::ATTACK),
            3 => Ok(Self::DISMISS),
            v => Err(crate::errors::EnumError::new("PetCommandState", v as u32),)
        }
    }
}

