use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Gender {
    MALE,
    FEMALE,
    NONE,
}

impl Gender {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::MALE => 0x0,
            Self::FEMALE => 0x1,
            Self::NONE => 0x2,
        }
    }

}

impl Default for Gender {
    fn default() -> Self {
        Self::MALE
    }
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MALE => f.write_str("MALE"),
            Self::FEMALE => f.write_str("FEMALE"),
            Self::NONE => f.write_str("NONE"),
        }
    }
}

impl TryFrom<u8> for Gender {
    type Error = GenderError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::MALE),
            1 => Ok(Self::FEMALE),
            2 => Ok(Self::NONE),
            _ => Err(GenderError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct GenderError {
    pub value: u8,
}

impl GenderError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for GenderError {}
impl std::fmt::Display for GenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'Gender': '{}'", self.value))
    }
}

