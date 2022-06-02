use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_stable_result.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_stable_result.wowm#L3):
/// ```text
/// enum StableResult : u8 {
///     ERR_MONEY = 0x01;
///     ERR_STABLE = 0x06;
///     SUCCESS_STABLE = 0x08;
///     SUCCESS_UNSTABLE = 0x09;
///     SUCCESS_BUY_SLOT = 0x0A;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum StableResult {
    /// # Comment
    /// 
    /// you don't have enough money
    ERR_MONEY,
    /// # Comment
    /// 
    /// currently used in most fail cases
    ERR_STABLE,
    /// # Comment
    /// 
    /// table success
    SUCCESS_STABLE,
    /// # Comment
    /// 
    /// unstable/swap success
    SUCCESS_UNSTABLE,
    /// # Comment
    /// 
    /// buy slot success
    SUCCESS_BUY_SLOT,
}

impl StableResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::ERR_MONEY => 0x1,
            Self::ERR_STABLE => 0x6,
            Self::SUCCESS_STABLE => 0x8,
            Self::SUCCESS_UNSTABLE => 0x9,
            Self::SUCCESS_BUY_SLOT => 0xa,
        }
    }

}

impl Default for StableResult {
    fn default() -> Self {
        Self::ERR_MONEY
    }
}

impl std::fmt::Display for StableResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ERR_MONEY => f.write_str("ERR_MONEY"),
            Self::ERR_STABLE => f.write_str("ERR_STABLE"),
            Self::SUCCESS_STABLE => f.write_str("SUCCESS_STABLE"),
            Self::SUCCESS_UNSTABLE => f.write_str("SUCCESS_UNSTABLE"),
            Self::SUCCESS_BUY_SLOT => f.write_str("SUCCESS_BUY_SLOT"),
        }
    }
}

impl TryFrom<u8> for StableResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::ERR_MONEY),
            6 => Ok(Self::ERR_STABLE),
            8 => Ok(Self::SUCCESS_STABLE),
            9 => Ok(Self::SUCCESS_UNSTABLE),
            10 => Ok(Self::SUCCESS_BUY_SLOT),
            v => Err(crate::errors::EnumError::new("StableResult", v as u32),)
        }
    }
}

