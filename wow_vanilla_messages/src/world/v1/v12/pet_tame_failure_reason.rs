use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PetTameFailureReason {
    INVALIDCREATURE,
    TOOMANY,
    CREATUREALREADYOWNED,
    NOTTAMEABLE,
    ANOTHERSUMMONACTIVE,
    UNITSCANTTAME,
    NOPETAVAILABLE,
    INTERNALERROR,
    TOOHIGHLEVEL,
    DEAD,
    NOTDEAD,
    UNKNOWNERROR,
}

impl PetTameFailureReason {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::INVALIDCREATURE => 0x1,
            Self::TOOMANY => 0x2,
            Self::CREATUREALREADYOWNED => 0x3,
            Self::NOTTAMEABLE => 0x4,
            Self::ANOTHERSUMMONACTIVE => 0x5,
            Self::UNITSCANTTAME => 0x6,
            Self::NOPETAVAILABLE => 0x7,
            Self::INTERNALERROR => 0x8,
            Self::TOOHIGHLEVEL => 0x9,
            Self::DEAD => 0xa,
            Self::NOTDEAD => 0xb,
            Self::UNKNOWNERROR => 0xc,
        }
    }

}

impl Default for PetTameFailureReason {
    fn default() -> Self {
        Self::INVALIDCREATURE
    }
}

impl std::fmt::Display for PetTameFailureReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::INVALIDCREATURE => f.write_str("INVALIDCREATURE"),
            Self::TOOMANY => f.write_str("TOOMANY"),
            Self::CREATUREALREADYOWNED => f.write_str("CREATUREALREADYOWNED"),
            Self::NOTTAMEABLE => f.write_str("NOTTAMEABLE"),
            Self::ANOTHERSUMMONACTIVE => f.write_str("ANOTHERSUMMONACTIVE"),
            Self::UNITSCANTTAME => f.write_str("UNITSCANTTAME"),
            Self::NOPETAVAILABLE => f.write_str("NOPETAVAILABLE"),
            Self::INTERNALERROR => f.write_str("INTERNALERROR"),
            Self::TOOHIGHLEVEL => f.write_str("TOOHIGHLEVEL"),
            Self::DEAD => f.write_str("DEAD"),
            Self::NOTDEAD => f.write_str("NOTDEAD"),
            Self::UNKNOWNERROR => f.write_str("UNKNOWNERROR"),
        }
    }
}

impl TryFrom<u8> for PetTameFailureReason {
    type Error = PetTameFailureReasonError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::INVALIDCREATURE),
            2 => Ok(Self::TOOMANY),
            3 => Ok(Self::CREATUREALREADYOWNED),
            4 => Ok(Self::NOTTAMEABLE),
            5 => Ok(Self::ANOTHERSUMMONACTIVE),
            6 => Ok(Self::UNITSCANTTAME),
            7 => Ok(Self::NOPETAVAILABLE),
            8 => Ok(Self::INTERNALERROR),
            9 => Ok(Self::TOOHIGHLEVEL),
            10 => Ok(Self::DEAD),
            11 => Ok(Self::NOTDEAD),
            12 => Ok(Self::UNKNOWNERROR),
            _ => Err(PetTameFailureReasonError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct PetTameFailureReasonError {
    value: u8,
}

impl PetTameFailureReasonError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for PetTameFailureReasonError {}
impl std::fmt::Display for PetTameFailureReasonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'PetTameFailureReason': '{}'", self.value))
    }
}

