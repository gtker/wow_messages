use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum RaidTargetIndex {
    UNKNOWN0,
    UNKNOWN1,
    UNKNOWN2,
    UNKNOWN3,
    UNKNOWN4,
    UNKNOWN5,
    UNKNOWN6,
    UNKNOWN7,
    UNKNOWN8,
    REQUEST_ICONS,
}

impl RaidTargetIndex {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::UNKNOWN0 => 0x0,
            Self::UNKNOWN1 => 0x1,
            Self::UNKNOWN2 => 0x2,
            Self::UNKNOWN3 => 0x3,
            Self::UNKNOWN4 => 0x4,
            Self::UNKNOWN5 => 0x5,
            Self::UNKNOWN6 => 0x6,
            Self::UNKNOWN7 => 0x7,
            Self::UNKNOWN8 => 0x8,
            Self::REQUEST_ICONS => 0xff,
        }
    }

}

impl Default for RaidTargetIndex {
    fn default() -> Self {
        Self::UNKNOWN0
    }
}

impl std::fmt::Display for RaidTargetIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UNKNOWN0 => f.write_str("UNKNOWN0"),
            Self::UNKNOWN1 => f.write_str("UNKNOWN1"),
            Self::UNKNOWN2 => f.write_str("UNKNOWN2"),
            Self::UNKNOWN3 => f.write_str("UNKNOWN3"),
            Self::UNKNOWN4 => f.write_str("UNKNOWN4"),
            Self::UNKNOWN5 => f.write_str("UNKNOWN5"),
            Self::UNKNOWN6 => f.write_str("UNKNOWN6"),
            Self::UNKNOWN7 => f.write_str("UNKNOWN7"),
            Self::UNKNOWN8 => f.write_str("UNKNOWN8"),
            Self::REQUEST_ICONS => f.write_str("REQUEST_ICONS"),
        }
    }
}

impl TryFrom<u8> for RaidTargetIndex {
    type Error = RaidTargetIndexError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::UNKNOWN0),
            1 => Ok(Self::UNKNOWN1),
            2 => Ok(Self::UNKNOWN2),
            3 => Ok(Self::UNKNOWN3),
            4 => Ok(Self::UNKNOWN4),
            5 => Ok(Self::UNKNOWN5),
            6 => Ok(Self::UNKNOWN6),
            7 => Ok(Self::UNKNOWN7),
            8 => Ok(Self::UNKNOWN8),
            255 => Ok(Self::REQUEST_ICONS),
            _ => Err(RaidTargetIndexError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct RaidTargetIndexError {
    pub value: u8,
}

impl RaidTargetIndexError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for RaidTargetIndexError {}
impl std::fmt::Display for RaidTargetIndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'RaidTargetIndex': '{}'", self.value))
    }
}

