use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum InstanceResetFailedReason {
    GENERAL,
    OFFLINE,
    ZONING,
    SILENTLY,
}

impl InstanceResetFailedReason {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::GENERAL => 0x0,
            Self::OFFLINE => 0x1,
            Self::ZONING => 0x2,
            Self::SILENTLY => 0x3,
        }
    }

}

impl Default for InstanceResetFailedReason {
    fn default() -> Self {
        Self::GENERAL
    }
}

impl std::fmt::Display for InstanceResetFailedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GENERAL => f.write_str("GENERAL"),
            Self::OFFLINE => f.write_str("OFFLINE"),
            Self::ZONING => f.write_str("ZONING"),
            Self::SILENTLY => f.write_str("SILENTLY"),
        }
    }
}

impl TryFrom<u8> for InstanceResetFailedReason {
    type Error = InstanceResetFailedReasonError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::GENERAL),
            1 => Ok(Self::OFFLINE),
            2 => Ok(Self::ZONING),
            3 => Ok(Self::SILENTLY),
            _ => Err(InstanceResetFailedReasonError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct InstanceResetFailedReasonError {
    value: u8,
}

impl InstanceResetFailedReasonError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for InstanceResetFailedReasonError {}
impl std::fmt::Display for InstanceResetFailedReasonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'InstanceResetFailedReason': '{}'", self.value))
    }
}

