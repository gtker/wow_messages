use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum NewItemChatAlert {
    DO_NOT_SHOW,
    SHOW,
}

impl NewItemChatAlert {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::DO_NOT_SHOW => 0x0,
            Self::SHOW => 0x1,
        }
    }

}

impl ConstantSized for NewItemChatAlert {}

impl MaximumPossibleSized for NewItemChatAlert {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for NewItemChatAlert {
    fn default() -> Self {
        Self::DO_NOT_SHOW
    }
}

impl std::fmt::Display for NewItemChatAlert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DO_NOT_SHOW => f.write_str("DO_NOT_SHOW"),
            Self::SHOW => f.write_str("SHOW"),
        }
    }
}

impl TryFrom<u32> for NewItemChatAlert {
    type Error = NewItemChatAlertError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DO_NOT_SHOW),
            1 => Ok(Self::SHOW),
            _ => Err(NewItemChatAlertError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct NewItemChatAlertError {
    value: u32,
}

impl NewItemChatAlertError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::error::Error for NewItemChatAlertError {}
impl std::fmt::Display for NewItemChatAlertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'NewItemChatAlert': '{}'", self.value))
    }
}

