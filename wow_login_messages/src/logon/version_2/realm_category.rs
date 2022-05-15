use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum RealmCategory {
    DEFAULT,
    ONE,
    TWO,
    THREE,
    FIVE,
}

impl RealmCategory {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::DEFAULT => 0x0,
            Self::ONE => 0x1,
            Self::TWO => 0x2,
            Self::THREE => 0x3,
            Self::FIVE => 0x5,
        }
    }

}

impl Default for RealmCategory {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl std::fmt::Display for RealmCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DEFAULT => f.write_str("DEFAULT"),
            Self::ONE => f.write_str("ONE"),
            Self::TWO => f.write_str("TWO"),
            Self::THREE => f.write_str("THREE"),
            Self::FIVE => f.write_str("FIVE"),
        }
    }
}

impl TryFrom<u8> for RealmCategory {
    type Error = RealmCategoryError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DEFAULT),
            1 => Ok(Self::ONE),
            2 => Ok(Self::TWO),
            3 => Ok(Self::THREE),
            5 => Ok(Self::FIVE),
            _ => Err(RealmCategoryError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct RealmCategoryError {
    value: u8,
}

impl RealmCategoryError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for RealmCategoryError {}
impl std::fmt::Display for RealmCategoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'RealmCategory': '{}'", self.value))
    }
}

