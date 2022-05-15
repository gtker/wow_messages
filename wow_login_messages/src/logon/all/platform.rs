use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Platform {
    X86,
    PPC,
    OTHER(u32),
}

impl Platform {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::X86 => 0x783836,
            Self::PPC => 0x505043,
            Self::OTHER(v) => *v,
        }
    }

}

impl ConstantSized for Platform {}

impl MaximumPossibleSized for Platform {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for Platform {
    fn default() -> Self {
        Self::X86
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X86 => f.write_str("X86"),
            Self::PPC => f.write_str("PPC"),
            Self::OTHER(v) => f.write_fmt(format_args!("OTHER({})", v)),
        }
    }
}

impl From<u32> for Platform {
    fn from(value: u32) -> Self {
        match value {
            7878710 => Self::X86,
            5263427 => Self::PPC,
            v => Self::OTHER(v)
        }
    }
}

