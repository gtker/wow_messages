use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PetTalkReason {
    SPECIAL_SPELL,
    ATTACK,
}

impl PetTalkReason {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::SPECIAL_SPELL => 0x0,
            Self::ATTACK => 0x1,
        }
    }

}

impl Default for PetTalkReason {
    fn default() -> Self {
        Self::SPECIAL_SPELL
    }
}

impl std::fmt::Display for PetTalkReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SPECIAL_SPELL => f.write_str("SPECIAL_SPELL"),
            Self::ATTACK => f.write_str("ATTACK"),
        }
    }
}

impl TryFrom<u32> for PetTalkReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SPECIAL_SPELL),
            1 => Ok(Self::ATTACK),
            v => Err(crate::errors::EnumError::new("PetTalkReason", v as u32),)
        }
    }
}

