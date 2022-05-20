use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Os {
    WINDOWS,
    OSX,
    OTHER(u32),
}

impl Os {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::WINDOWS => 0x57696e,
            Self::OSX => 0x4f5358,
            Self::OTHER(v) => *v,
        }
    }

}

impl Default for Os {
    fn default() -> Self {
        Self::WINDOWS
    }
}

impl std::fmt::Display for Os {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WINDOWS => f.write_str("WINDOWS"),
            Self::OSX => f.write_str("OSX"),
            Self::OTHER(v) => f.write_fmt(format_args!("OTHER({})", v)),
        }
    }
}

impl From<u32> for Os {
    fn from(value: u32) -> Self {
        match value {
            5728622 => Self::WINDOWS,
            5198680 => Self::OSX,
            v => Self::OTHER(v)
        }
    }
}

