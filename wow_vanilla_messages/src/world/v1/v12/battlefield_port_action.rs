use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum BattlefieldPortAction {
    LEAVE_QUEUE,
    ENTER_BATTLE,
}

impl BattlefieldPortAction {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::LEAVE_QUEUE => 0x0,
            Self::ENTER_BATTLE => 0x1,
        }
    }

}

impl Default for BattlefieldPortAction {
    fn default() -> Self {
        Self::LEAVE_QUEUE
    }
}

impl std::fmt::Display for BattlefieldPortAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LEAVE_QUEUE => f.write_str("LEAVE_QUEUE"),
            Self::ENTER_BATTLE => f.write_str("ENTER_BATTLE"),
        }
    }
}

impl TryFrom<u8> for BattlefieldPortAction {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::LEAVE_QUEUE),
            1 => Ok(Self::ENTER_BATTLE),
            v => Err(crate::errors::EnumError::new("BattlefieldPortAction", v as u32),)
        }
    }
}

