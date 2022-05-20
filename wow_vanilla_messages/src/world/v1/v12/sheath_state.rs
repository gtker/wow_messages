use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum SheathState {
    UNARMED,
    MELEE,
    RANGED,
}

impl SheathState {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::UNARMED => 0x0,
            Self::MELEE => 0x1,
            Self::RANGED => 0x2,
        }
    }

}

impl Default for SheathState {
    fn default() -> Self {
        Self::UNARMED
    }
}

impl std::fmt::Display for SheathState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UNARMED => f.write_str("UNARMED"),
            Self::MELEE => f.write_str("MELEE"),
            Self::RANGED => f.write_str("RANGED"),
        }
    }
}

impl TryFrom<u8> for SheathState {
    type Error = SheathStateError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::UNARMED),
            1 => Ok(Self::MELEE),
            2 => Ok(Self::RANGED),
            _ => Err(SheathStateError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct SheathStateError {
    pub value: u8,
}

impl SheathStateError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for SheathStateError {}
impl std::fmt::Display for SheathStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'SheathState': '{}'", self.value))
    }
}

