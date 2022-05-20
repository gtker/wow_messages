use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

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
    type Error = PetReactStateError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PASSIVE),
            1 => Ok(Self::DEFENSIVE),
            2 => Ok(Self::AGGRESSIVE),
            _ => Err(PetReactStateError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct PetReactStateError {
    pub value: u8,
}

impl PetReactStateError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for PetReactStateError {}
impl std::fmt::Display for PetReactStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'PetReactState': '{}'", self.value))
    }
}

