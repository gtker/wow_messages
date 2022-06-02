use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/pet_common.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/pet_common.wowm#L3):
/// ```text
/// enum PetReactState : u8 {
///     PASSIVE = 0;
///     DEFENSIVE = 1;
///     AGGRESSIVE = 2;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PetReactState {
    PASSIVE,
    DEFENSIVE,
    AGGRESSIVE,
}

impl PetReactState {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::PASSIVE => 0x0,
            Self::DEFENSIVE => 0x1,
            Self::AGGRESSIVE => 0x2,
        }
    }

}

impl Default for PetReactState {
    fn default() -> Self {
        Self::PASSIVE
    }
}

impl std::fmt::Display for PetReactState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PASSIVE => f.write_str("PASSIVE"),
            Self::DEFENSIVE => f.write_str("DEFENSIVE"),
            Self::AGGRESSIVE => f.write_str("AGGRESSIVE"),
        }
    }
}

impl TryFrom<u8> for PetReactState {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PASSIVE),
            1 => Ok(Self::DEFENSIVE),
            2 => Ok(Self::AGGRESSIVE),
            v => Err(crate::errors::EnumError::new("PetReactState", v as u32),)
        }
    }
}

