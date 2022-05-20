use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ActivateTaxiReply {
    OK,
    UNSPECIFIEDSERVERERROR,
    NOSUCHPATH,
    NOTENOUGHMONEY,
    TOOFARAWAY,
    NOVENDORNEARBY,
    NOTVISITED,
    PLAYERBUSY,
    PLAYERALREADYMOUNTED,
    PLAYERSHAPESHIFTED,
    PLAYERMOVING,
    SAMENODE,
    NOTSTANDING,
}

impl ActivateTaxiReply {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::OK => 0x0,
            Self::UNSPECIFIEDSERVERERROR => 0x1,
            Self::NOSUCHPATH => 0x2,
            Self::NOTENOUGHMONEY => 0x3,
            Self::TOOFARAWAY => 0x4,
            Self::NOVENDORNEARBY => 0x5,
            Self::NOTVISITED => 0x6,
            Self::PLAYERBUSY => 0x7,
            Self::PLAYERALREADYMOUNTED => 0x8,
            Self::PLAYERSHAPESHIFTED => 0x9,
            Self::PLAYERMOVING => 0xa,
            Self::SAMENODE => 0xb,
            Self::NOTSTANDING => 0xc,
        }
    }

}

impl Default for ActivateTaxiReply {
    fn default() -> Self {
        Self::OK
    }
}

impl std::fmt::Display for ActivateTaxiReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OK => f.write_str("OK"),
            Self::UNSPECIFIEDSERVERERROR => f.write_str("UNSPECIFIEDSERVERERROR"),
            Self::NOSUCHPATH => f.write_str("NOSUCHPATH"),
            Self::NOTENOUGHMONEY => f.write_str("NOTENOUGHMONEY"),
            Self::TOOFARAWAY => f.write_str("TOOFARAWAY"),
            Self::NOVENDORNEARBY => f.write_str("NOVENDORNEARBY"),
            Self::NOTVISITED => f.write_str("NOTVISITED"),
            Self::PLAYERBUSY => f.write_str("PLAYERBUSY"),
            Self::PLAYERALREADYMOUNTED => f.write_str("PLAYERALREADYMOUNTED"),
            Self::PLAYERSHAPESHIFTED => f.write_str("PLAYERSHAPESHIFTED"),
            Self::PLAYERMOVING => f.write_str("PLAYERMOVING"),
            Self::SAMENODE => f.write_str("SAMENODE"),
            Self::NOTSTANDING => f.write_str("NOTSTANDING"),
        }
    }
}

impl TryFrom<u32> for ActivateTaxiReply {
    type Error = ActivateTaxiReplyError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OK),
            1 => Ok(Self::UNSPECIFIEDSERVERERROR),
            2 => Ok(Self::NOSUCHPATH),
            3 => Ok(Self::NOTENOUGHMONEY),
            4 => Ok(Self::TOOFARAWAY),
            5 => Ok(Self::NOVENDORNEARBY),
            6 => Ok(Self::NOTVISITED),
            7 => Ok(Self::PLAYERBUSY),
            8 => Ok(Self::PLAYERALREADYMOUNTED),
            9 => Ok(Self::PLAYERSHAPESHIFTED),
            10 => Ok(Self::PLAYERMOVING),
            11 => Ok(Self::SAMENODE),
            12 => Ok(Self::NOTSTANDING),
            _ => Err(ActivateTaxiReplyError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct ActivateTaxiReplyError {
    pub value: u32,
}

impl ActivateTaxiReplyError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::error::Error for ActivateTaxiReplyError {}
impl std::fmt::Display for ActivateTaxiReplyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'ActivateTaxiReply': '{}'", self.value))
    }
}

